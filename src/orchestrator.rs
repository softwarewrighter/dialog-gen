use crate::config::{DialogConfig, Speaker};
use crate::error::Result;
use crate::ollama::{ChatMessage, OllamaClient};
use std::time::Duration;

pub struct DialogOrchestrator {
    ollama: OllamaClient,
    config: DialogConfig,
}

#[derive(Debug, Clone)]
pub struct DialogExchange {
    pub speaker: String,
    pub content: String,
}

/// Metadata about the generation run
#[derive(Debug)]
pub struct GenerationMetadata {
    pub model: String,
    pub turns: usize,
    pub temperature: f32,
    pub total_prompt_tokens: u64,
    pub total_completion_tokens: u64,
    pub total_wall_time: Duration,
    pub avg_tokens_per_second: f64,
}

#[derive(Debug)]
pub struct GeneratedDialog {
    pub exchanges: Vec<DialogExchange>,
    pub metadata: GenerationMetadata,
}

impl DialogOrchestrator {
    pub fn new(ollama: OllamaClient, config: DialogConfig) -> Self {
        Self { ollama, config }
    }

    /// Build system prompt for a speaker
    fn build_system_prompt(&self, speaker: &Speaker, other_speaker: &Speaker) -> String {
        format!(
            r#"You are {name} talking to {other_name}.

{name}: {background} {personality} {speaking_style}

Scene: {scene}. {setting}
{notes}

RESPOND WITH EXACTLY ONE SHORT SENTENCE. Either a statement OR a question, never both. No followup. Just react to what {other_name} said."#,
            name = speaker.name,
            other_name = other_speaker.name,
            background = speaker.background,
            personality = speaker.personality,
            speaking_style = speaker.speaking_style,
            scene = self.config.directions.scene_name,
            setting = self.config.directions.setting,
            notes = self.config.directions.notes,
        )
    }

    /// Get the other speaker
    fn get_other_speaker(&self, speaker: &Speaker) -> &Speaker {
        if speaker.name == self.config.speaker1.name {
            &self.config.speaker2
        } else {
            &self.config.speaker1
        }
    }

    /// Build conversation history as alternating user/assistant messages
    /// from the perspective of the current speaker
    fn build_conversation_history(
        &self,
        current_speaker: &Speaker,
        exchanges: &[DialogExchange],
    ) -> Vec<ChatMessage> {
        let mut messages = Vec::new();

        for exchange in exchanges {
            let role = if exchange.speaker == current_speaker.name {
                "assistant" // Lines from current speaker are "assistant" (what I said)
            } else {
                "user" // Lines from other speaker are "user" (what they said to me)
            };

            messages.push(ChatMessage {
                role: role.to_string(),
                content: exchange.content.clone(),
            });
        }

        messages
    }

    /// Determine which speaker should go next based on who spoke last
    fn get_next_speaker(&self, last_speaker: Option<&str>) -> &Speaker {
        match last_speaker {
            Some(name) if name == self.config.speaker1.name => &self.config.speaker2,
            Some(name) if name == self.config.speaker2.name => &self.config.speaker1,
            // Default to speaker2 responding first (since prompt usually has speaker1 starting)
            _ => &self.config.speaker2,
        }
    }

    /// Generate the complete dialog
    pub async fn generate(&self, verbose: bool) -> Result<GeneratedDialog> {
        let mut exchanges: Vec<DialogExchange> = self
            .config
            .initial_lines
            .iter()
            .map(|l| DialogExchange {
                speaker: l.speaker.clone(),
                content: l.content.clone(),
            })
            .collect();

        // Determine starting speaker (opposite of last in prompt)
        let last_speaker = self.config.initial_lines.last().map(|l| l.speaker.as_str());

        if verbose {
            eprintln!("Initial dialog:");
            for line in &self.config.initial_lines {
                eprintln!("  {}: {}", line.speaker, line.content);
            }
            eprintln!("\nGenerating {} turns...\n", self.config.scene.turns);
        }

        let mut current_speaker = self.get_next_speaker(last_speaker);

        // Track aggregate stats
        let mut total_prompt_tokens: u64 = 0;
        let mut total_completion_tokens: u64 = 0;
        let mut total_wall_time = Duration::ZERO;
        let mut total_eval_ns: u64 = 0;

        for turn in 0..self.config.scene.turns {
            if verbose {
                eprint!(
                    "Turn {}/{}: {} ... ",
                    turn + 1,
                    self.config.scene.turns,
                    current_speaker.name
                );
            }

            let other_speaker = self.get_other_speaker(current_speaker);
            let system_prompt = self.build_system_prompt(current_speaker, other_speaker);
            let messages = self.build_conversation_history(current_speaker, &exchanges);

            let result = self
                .ollama
                .chat(&system_prompt, &messages, self.config.scene.temperature)
                .await?;

            // Accumulate stats
            total_prompt_tokens += result.stats.prompt_tokens;
            total_completion_tokens += result.stats.completion_tokens;
            total_wall_time += result.stats.wall_time;
            total_eval_ns += result.stats.eval_duration_ns;

            // Clean up response (remove any accidental name prefix)
            let cleaned_response = self.clean_response(&result.content, &current_speaker.name);

            if verbose {
                eprintln!("{}", cleaned_response);
            }

            exchanges.push(DialogExchange {
                speaker: current_speaker.name.clone(),
                content: cleaned_response,
            });

            // Alternate speakers
            current_speaker = if current_speaker.name == self.config.speaker1.name {
                &self.config.speaker2
            } else {
                &self.config.speaker1
            };
        }

        let avg_tokens_per_second = if total_eval_ns > 0 {
            (total_completion_tokens as f64) / (total_eval_ns as f64 / 1_000_000_000.0)
        } else {
            0.0
        };

        let metadata = GenerationMetadata {
            model: self.ollama.model().to_string(),
            turns: self.config.scene.turns,
            temperature: self.config.scene.temperature,
            total_prompt_tokens,
            total_completion_tokens,
            total_wall_time,
            avg_tokens_per_second,
        };

        Ok(GeneratedDialog {
            exchanges,
            metadata,
        })
    }

    /// Clean up LLM response - remove name prefix, truncate if needed
    fn clean_response(&self, response: &str, speaker_name: &str) -> String {
        let mut result = response.trim().to_string();

        // Check if response starts with speaker name prefix
        let prefixes = [
            format!("{}:", speaker_name),
            format!("{} :", speaker_name),
            format!("{}:", speaker_name.to_uppercase()),
            format!("{}:", speaker_name.to_lowercase()),
        ];

        for prefix in &prefixes {
            if let Some(stripped) = result.strip_prefix(prefix) {
                result = stripped.trim().to_string();
                break;
            }
        }

        // Remove wrapping quotes
        result = result.trim_matches('"').trim().to_string();

        // Take only the first paragraph if multiple paragraphs
        if let Some(pos) = result.find("\n\n") {
            result = result[..pos].trim().to_string();
        }

        // If still too long (>500 chars), truncate at sentence boundary
        if result.len() > 500 {
            if let Some(pos) = result[..500].rfind(". ") {
                result = result[..=pos].to_string();
            } else if let Some(pos) = result[..500].rfind('.') {
                result = result[..=pos].to_string();
            }
        }

        result
    }
}

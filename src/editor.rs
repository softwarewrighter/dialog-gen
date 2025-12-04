use crate::error::Result;
use crate::ollama::{ChatMessage, OllamaClient};
use crate::orchestrator::{DialogExchange, GeneratedDialog, GenerationMetadata};

const PODCAST_EDITOR_SYSTEM_PROMPT: &str = r#"You are an expert podcast editor with years of experience making conversations more engaging and natural-sounding.

Your goal is to edit podcast scripts so that:
- Listeners stay engaged and come back for more episodes
- The conversation flows naturally with good pacing
- Each speaker has a distinct, consistent voice
- The dialog feels authentic, not scripted
- Awkward phrasings are smoothed out
- Repetitive content is trimmed or varied
- The energy and momentum build appropriately

Edit the following podcast script. Preserve the speaker names and format (SPEAKER: dialog).
Make it tighter, more engaging, and more natural. Keep the same general content and meaning, but improve the delivery."#;

pub struct PodcastEditor {
    ollama: OllamaClient,
}

impl PodcastEditor {
    pub fn new(ollama: OllamaClient) -> Self {
        Self { ollama }
    }

    /// Edit a generated dialog to improve quality
    pub async fn edit(&self, dialog: &GeneratedDialog, verbose: bool) -> Result<GeneratedDialog> {
        if verbose {
            eprintln!("\n--- Editing podcast script ---\n");
        }

        // Format the dialog as a script
        let mut script = String::new();
        for exchange in &dialog.exchanges {
            script.push_str(&format!("{}: {}\n\n", exchange.speaker, exchange.content));
        }

        let user_prompt = format!("Edit this podcast script:\n\n{}", script.trim());

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: user_prompt,
        }];

        if verbose {
            eprint!("Sending to editor... ");
        }

        let result = self
            .ollama
            .chat(PODCAST_EDITOR_SYSTEM_PROMPT, &messages, 0.7)
            .await?;

        if verbose {
            eprintln!("done ({:.1}s)", result.stats.wall_time.as_secs_f64());
        }

        // Parse the edited script back into exchanges
        let edited_exchanges = self.parse_script(&result.content);

        // Update metadata to reflect editing
        let metadata = GenerationMetadata {
            model: format!("{} (edited)", dialog.metadata.model),
            turns: edited_exchanges.len(),
            temperature: dialog.metadata.temperature,
            total_prompt_tokens: dialog.metadata.total_prompt_tokens + result.stats.prompt_tokens,
            total_completion_tokens: dialog.metadata.total_completion_tokens
                + result.stats.completion_tokens,
            total_wall_time: dialog.metadata.total_wall_time + result.stats.wall_time,
            avg_tokens_per_second: dialog.metadata.avg_tokens_per_second, // Keep original
        };

        Ok(GeneratedDialog {
            exchanges: edited_exchanges,
            metadata,
        })
    }

    /// Parse a script back into dialog exchanges
    fn parse_script(&self, script: &str) -> Vec<DialogExchange> {
        let mut exchanges = Vec::new();

        for line in script.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Look for "Speaker: content" pattern
            if let Some(colon_pos) = trimmed.find(':') {
                let speaker = trimmed[..colon_pos].trim();
                let content = trimmed[colon_pos + 1..].trim();

                // Validate it looks like a speaker name (starts with uppercase, reasonable length)
                if !speaker.is_empty()
                    && speaker.len() < 50
                    && speaker.chars().next().is_some_and(|c| c.is_uppercase())
                    && !content.is_empty()
                {
                    exchanges.push(DialogExchange {
                        speaker: speaker.to_string(),
                        content: content.to_string(),
                    });
                }
            }
        }

        exchanges
    }
}

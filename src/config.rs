use crate::error::{DialogGenError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Speaker character definition
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Speaker {
    pub name: String,
    pub background: String,
    pub personality: String,
    pub motivations: String,
    pub speaking_style: String,
}

/// Scene configuration
#[derive(Debug, Clone)]
pub struct Scene {
    pub turns: usize,
    pub model: Option<String>,
    pub temperature: f32,
}

/// Director's notes
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Directions {
    pub scene_name: String,
    pub setting: String,
    pub mood: String,
    pub goal: String,
    pub notes: String,
}

/// Parsed dialog line from prompt
#[derive(Debug, Clone)]
pub struct DialogLine {
    pub speaker: String,
    pub content: String,
}

/// Complete configuration for a dialog generation run
#[derive(Debug)]
pub struct DialogConfig {
    pub speaker1: Speaker,
    pub speaker2: Speaker,
    pub directions: Directions,
    pub initial_lines: Vec<DialogLine>,
    pub scene: Scene,
}

impl DialogConfig {
    pub fn load(input_dir: &Path) -> Result<Self> {
        let speaker1 = Self::load_speaker(&input_dir.join("speaker1.txt"), "speaker1.txt")?;
        let speaker2 = Self::load_speaker(&input_dir.join("speaker2.txt"), "speaker2.txt")?;
        let directions = Self::load_directions(&input_dir.join("directions.txt"))?;
        let initial_lines = Self::load_prompt(&input_dir.join("prompt.txt"))?;
        let scene = Self::load_scene(&input_dir.join("scene.txt"))?;

        Ok(DialogConfig {
            speaker1,
            speaker2,
            directions,
            initial_lines,
            scene,
        })
    }

    fn load_speaker(path: &Path, filename: &str) -> Result<Speaker> {
        let content = fs::read_to_string(path)
            .map_err(|_| DialogGenError::MissingFile(filename.to_string()))?;

        let fields = Self::parse_key_value(&content);

        Ok(Speaker {
            name: fields.get("name").cloned().unwrap_or_default(),
            background: fields.get("background").cloned().unwrap_or_default(),
            personality: fields.get("personality").cloned().unwrap_or_default(),
            motivations: fields.get("motivations").cloned().unwrap_or_default(),
            speaking_style: fields.get("speaking style").cloned().unwrap_or_default(),
        })
    }

    fn load_directions(path: &Path) -> Result<Directions> {
        let content = fs::read_to_string(path)
            .map_err(|_| DialogGenError::MissingFile("directions.txt".to_string()))?;

        let fields = Self::parse_key_value(&content);

        Ok(Directions {
            scene_name: fields.get("scene").cloned().unwrap_or_default(),
            setting: fields.get("setting").cloned().unwrap_or_default(),
            mood: fields.get("mood").cloned().unwrap_or_default(),
            goal: fields.get("goal").cloned().unwrap_or_default(),
            notes: fields.get("notes").cloned().unwrap_or_default(),
        })
    }

    fn load_scene(path: &Path) -> Result<Scene> {
        let content = fs::read_to_string(path)
            .map_err(|_| DialogGenError::MissingFile("scene.txt".to_string()))?;

        let fields = Self::parse_key_value(&content);

        let turns = fields
            .get("turns")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);

        let temperature = fields
            .get("temperature")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.7);

        let model = fields.get("model").cloned();

        Ok(Scene {
            turns,
            model,
            temperature,
        })
    }

    fn load_prompt(path: &Path) -> Result<Vec<DialogLine>> {
        let content = fs::read_to_string(path)
            .map_err(|_| DialogGenError::MissingFile("prompt.txt".to_string()))?;

        let mut lines = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check if line starts with a speaker name (NAME: format)
            if let Some(colon_pos) = trimmed.find(':') {
                let potential_name = &trimmed[..colon_pos];
                // Speaker names are typically uppercase or Title Case, no spaces before colon
                if potential_name
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_whitespace())
                    && !potential_name.is_empty()
                    && potential_name
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_uppercase())
                {
                    let speaker = potential_name.trim().to_string();
                    let content = trimmed[colon_pos + 1..].trim().to_string();
                    if !content.is_empty() {
                        lines.push(DialogLine { speaker, content });
                    }
                }
            }
        }

        Ok(lines)
    }

    /// Parse key-value pairs from a file with format "Key: Value"
    /// Handles multi-line values (lines without colons are appended to previous value)
    fn parse_key_value(content: &str) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        let mut current_key: Option<String> = None;
        let mut current_value = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Check if this line starts a new key
            if let Some(colon_pos) = trimmed.find(':') {
                let potential_key = &trimmed[..colon_pos];
                // Keys should be simple words/phrases
                if potential_key
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_whitespace())
                    && !potential_key.is_empty()
                {
                    // Save previous key-value pair
                    if let Some(key) = current_key.take() {
                        fields.insert(key, current_value.trim().to_string());
                    }

                    current_key = Some(potential_key.to_lowercase());
                    current_value = trimmed[colon_pos + 1..].trim().to_string();
                    continue;
                }
            }

            // Append to current value if we have a key
            if current_key.is_some() && !trimmed.is_empty() {
                if !current_value.is_empty() {
                    current_value.push(' ');
                }
                current_value.push_str(trimmed);
            }
        }

        // Don't forget the last key-value pair
        if let Some(key) = current_key {
            fields.insert(key, current_value.trim().to_string());
        }

        fields
    }
}

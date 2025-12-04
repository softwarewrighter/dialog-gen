# Dialog-Gen Technical Design

## Module Design

### 1. Main Module (`src/main.rs`)

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "dialog-gen")]
#[command(about = "Generate AI-powered dialog between two characters")]
struct Cli {
    /// Input directory containing configuration files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory (defaults to input directory)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Ollama model to use
    #[arg(short, long, default_value = "mistral:7b")]
    model: String,

    /// Ollama server URL
    #[arg(long, default_value = "http://localhost:11434")]
    ollama_url: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    // ... orchestration logic
}
```

### 2. Configuration Module (`src/config.rs`)

```rust
/// Speaker character definition
pub struct Speaker {
    pub name: String,
    pub background: String,
    pub personality: String,
    pub motivations: String,
    pub speaking_style: String,
}

/// Scene configuration
pub struct Scene {
    pub turns: usize,
    pub model: Option<String>,
    pub temperature: Option<f32>,
}

/// Director's notes
pub struct Directions {
    pub scene_name: String,
    pub setting: String,
    pub mood: String,
    pub goal: String,
    pub notes: String,
}

/// Complete configuration for a dialog generation run
pub struct DialogConfig {
    pub speaker1: Speaker,
    pub speaker2: Speaker,
    pub directions: Directions,
    pub initial_prompt: String,
    pub scene: Scene,
}

impl DialogConfig {
    pub fn load(input_dir: &Path) -> Result<Self> {
        // Read and parse all configuration files
    }
}
```

### 3. Ollama Client Module (`src/ollama.rs`)

```rust
pub struct OllamaClient {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

pub struct ChatMessage {
    pub role: String,      // "system", "user", "assistant"
    pub content: String,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self { ... }

    /// Generate a response using the chat API
    pub async fn chat(
        &self,
        system_prompt: &str,
        messages: &[ChatMessage],
    ) -> Result<String> {
        // POST to /api/chat
        // Handle streaming response
        // Return final content
    }

    /// Check if Ollama server is available
    pub async fn health_check(&self) -> Result<bool> { ... }
}
```

### 4. Orchestrator Module (`src/orchestrator.rs`)

```rust
pub struct DialogOrchestrator {
    ollama: OllamaClient,
    config: DialogConfig,
}

pub struct GeneratedDialog {
    pub exchanges: Vec<DialogExchange>,
}

pub struct DialogExchange {
    pub speaker: String,
    pub content: String,
}

impl DialogOrchestrator {
    pub fn new(ollama: OllamaClient, config: DialogConfig) -> Self { ... }

    /// Build system prompt for a speaker
    fn build_system_prompt(&self, speaker: &Speaker) -> String {
        format!(
            "You are {}, a character in a scene.\n\n\
             Background: {}\n\
             Personality: {}\n\
             Motivations: {}\n\
             Speaking Style: {}\n\n\
             Director's Notes:\n\
             Scene: {}\n\
             Setting: {}\n\
             Mood: {}\n\
             Goal: {}\n\
             {}\n\n\
             Respond only with your character's next line of dialog. \
             Stay in character. Do not include stage directions or \
             your character's name prefix.",
            speaker.name,
            speaker.background,
            speaker.personality,
            speaker.motivations,
            speaker.speaking_style,
            self.config.directions.scene_name,
            self.config.directions.setting,
            self.config.directions.mood,
            self.config.directions.goal,
            self.config.directions.notes,
        )
    }

    /// Parse initial prompt to extract starting dialog
    fn parse_initial_prompt(&self) -> Vec<DialogExchange> {
        // Parse prompt.txt format
        // Return structured dialog lines
    }

    /// Generate the complete dialog
    pub async fn generate(&self) -> Result<GeneratedDialog> {
        let mut exchanges = self.parse_initial_prompt();
        let mut messages: Vec<ChatMessage> = vec![];

        // Convert initial exchanges to message format
        for exchange in &exchanges {
            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: format!("{}: {}", exchange.speaker, exchange.content),
            });
        }

        // Determine starting speaker (opposite of last in prompt)
        let mut current_speaker = if exchanges.last()
            .map(|e| e.speaker == self.config.speaker1.name)
            .unwrap_or(false)
        {
            &self.config.speaker2
        } else {
            &self.config.speaker1
        };

        // Generate dialog turns
        for _ in 0..self.config.scene.turns {
            let system_prompt = self.build_system_prompt(current_speaker);
            let response = self.ollama.chat(&system_prompt, &messages).await?;

            exchanges.push(DialogExchange {
                speaker: current_speaker.name.clone(),
                content: response.clone(),
            });

            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: format!("{}: {}", current_speaker.name, response),
            });

            // Alternate speakers
            current_speaker = if current_speaker.name == self.config.speaker1.name {
                &self.config.speaker2
            } else {
                &self.config.speaker1
            };
        }

        Ok(GeneratedDialog { exchanges })
    }
}
```

### 5. Output Module (`src/output.rs`)

```rust
pub struct OutputWriter {
    output_dir: PathBuf,
}

impl OutputWriter {
    pub fn new(output_dir: PathBuf) -> Self { ... }

    /// Find next available output file number
    fn next_file_number(&self) -> Result<usize> {
        // Scan for existing generated-dialogN.txt files
        // Return next available number
    }

    /// Write generated dialog to file
    pub fn write(&self, dialog: &GeneratedDialog) -> Result<PathBuf> {
        let num = self.next_file_number()?;
        let path = self.output_dir.join(format!("generated-dialog{}.txt", num));

        let mut content = String::new();
        for exchange in &dialog.exchanges {
            content.push_str(&format!("{}: {}\n\n", exchange.speaker, exchange.content));
        }

        fs::write(&path, content)?;
        Ok(path)
    }
}
```

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DialogGenError {
    #[error("Missing required file: {0}")]
    MissingFile(String),

    #[error("Invalid configuration in {file}: {message}")]
    InvalidConfig { file: String, message: String },

    #[error("Ollama server unavailable at {0}")]
    OllamaUnavailable(String),

    #[error("LLM generation failed: {0}")]
    GenerationFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
```

## API Interaction

### Ollama Chat API

```
POST http://localhost:11434/api/chat

Request:
{
    "model": "mistral:7b",
    "messages": [
        {"role": "system", "content": "..."},
        {"role": "user", "content": "..."},
        {"role": "assistant", "content": "..."}
    ],
    "stream": false,
    "options": {
        "temperature": 0.7
    }
}

Response:
{
    "model": "mistral:7b",
    "message": {
        "role": "assistant",
        "content": "Generated response..."
    },
    "done": true
}
```

## File Parsing

### Speaker File Format
Simple key-value parsing with multi-line support:
```
Name: John Smith
Background: A retired detective who...
(continues on next lines until next key)
Personality: Gruff but caring...
```

### Prompt File Format
- Lines starting with `SPEAKERNAME:` are dialog
- Other lines are context/narration
- First speaker line determines who spoke first

## Testing Strategy

1. **Unit Tests**: Each module has isolated tests
2. **Integration Tests**: Full pipeline with mock Ollama
3. **Manual Testing**: test-data/ directory with sample inputs

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
```

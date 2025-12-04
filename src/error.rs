use thiserror::Error;

#[derive(Error, Debug)]
pub enum DialogGenError {
    #[error("Missing required file: {0}")]
    MissingFile(String),

    #[error("Ollama server unavailable at {0}")]
    OllamaUnavailable(String),

    #[error("LLM generation failed: {0}")]
    GenerationFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, DialogGenError>;

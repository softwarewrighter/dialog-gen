mod config;
mod error;
mod ollama;
mod orchestrator;
mod output;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use config::DialogConfig;
use ollama::OllamaClient;
use orchestrator::DialogOrchestrator;
use output::OutputWriter;

#[derive(Parser)]
#[command(name = "dialog-gen")]
#[command(about = "Generate AI-powered dialog between two characters using local LLM")]
#[command(version)]
struct Cli {
    /// Input directory containing configuration files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory (defaults to input directory)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Ollama model to use (overrides scene.txt)
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

    // Validate input directory exists
    if !cli.input.exists() {
        anyhow::bail!("Input directory does not exist: {}", cli.input.display());
    }

    if !cli.input.is_dir() {
        anyhow::bail!("Input path is not a directory: {}", cli.input.display());
    }

    // Load configuration
    if cli.verbose {
        eprintln!("Loading configuration from: {}", cli.input.display());
    }

    let config = DialogConfig::load(&cli.input)?;

    if cli.verbose {
        eprintln!(
            "Loaded speakers: {} and {}",
            config.speaker1.name, config.speaker2.name
        );
        eprintln!(
            "Scene: {} ({} turns)",
            config.directions.scene_name, config.scene.turns
        );
    }

    // Determine model (CLI override > scene.txt > default)
    let model = if cli.model != "mistral:7b" {
        cli.model.clone()
    } else {
        config.scene.model.clone().unwrap_or(cli.model.clone())
    };

    // Create Ollama client
    let ollama = OllamaClient::new(&cli.ollama_url, &model);

    // Check Ollama server availability
    if cli.verbose {
        eprintln!(
            "Connecting to Ollama at {} with model {}...",
            cli.ollama_url, model
        );
    }

    if !ollama.health_check().await? {
        anyhow::bail!(
            "Ollama server not available at {}. Is Ollama running?",
            cli.ollama_url
        );
    }

    if cli.verbose {
        eprintln!("Ollama server connected.\n");
    }

    // Create orchestrator and generate dialog
    let orchestrator = DialogOrchestrator::new(ollama, config);
    let dialog = orchestrator.generate(cli.verbose).await?;

    // Write output
    let output_dir = cli.output.unwrap_or_else(|| cli.input.clone());
    let writer = OutputWriter::new(output_dir);
    let output_path = writer.write(&dialog)?;

    println!("\nDialog generated: {}", output_path.display());
    println!("\n--- Generated Dialog ---\n");

    for exchange in &dialog.exchanges {
        println!("{}: {}\n", exchange.speaker, exchange.content);
    }

    Ok(())
}

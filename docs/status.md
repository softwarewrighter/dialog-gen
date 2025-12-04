# Dialog-Gen Project Status

## Current Status: **v0.3.0 - Podcast Editor**

Last Updated: 2025-12-04

## Completed

- [x] Project initialization (Cargo.toml, Rust 2024 edition)
- [x] Documentation structure
- [x] Core implementation
  - [x] src/error.rs - Custom error types
  - [x] src/config.rs - Input file parsing
  - [x] src/ollama.rs - Ollama HTTP client with stats tracking
  - [x] src/orchestrator.rs - Dialog generation with metadata
  - [x] src/output.rs - Dialog + metadata file output
  - [x] src/main.rs - CLI integration
- [x] Demo scenarios with outputs
  - [x] prompt1 - AI debate (verbose, early iteration)
  - [x] prompt2 - VC pitch (verbose, early iteration)
  - [x] prompt3-mistral - Podcast chat with mistral:7b
  - [x] prompt3-llama - Podcast chat with llama3.2 (best results)
- [x] Output metadata (model, tokens, timing stats)
- [x] Code quality (fmt, clippy clean)
- [x] Podcast editor (--edit-podcast flag)
  - [x] src/editor.rs - LLM-powered dialog editing
  - [x] Edited output files (edited-podcast.txt, edited-metadata.txt)
  - [x] Demo with edited dialog (prompt3-llama)

## Not Started

- [ ] Unit tests
- [ ] Integration tests

## Development Environment

- **Rust Edition**: 2024
- **LLM Backend**: Ollama (local)
- **Recommended Model**: llama3.2:latest (best dialog quality)

## CLI Options

```
Usage: dialog-gen [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>            Input directory containing configuration files
  -o, --output <OUTPUT>          Output directory (defaults to input directory)
  -m, --model <MODEL>            Ollama model to use [default: mistral:7b]
      --ollama-url <OLLAMA_URL>  Ollama server URL [default: http://localhost:11434]
  -e, --edit-podcast             Apply podcast editor to improve dialog quality
  -v, --verbose                  Verbose output
  -h, --help                     Print help
  -V, --version                  Print version
```

## Model Comparison (prompt3 scenario)

| Model | Tokens/s | Style | Recommendation |
|-------|----------|-------|----------------|
| mistral:7b | 29.4 | Terse, choppy | Good for short exchanges |
| llama3.2 | 54.3 | Natural, flowing | Best for conversation |

## Output Files

Each run produces:
- `generated-dialogN.txt` - The conversation
- `output-metadataN.txt` - Model, tokens, timing stats

With `--edit-podcast`:
- `edited-podcast.txt` - Edited dialog
- `edited-metadata.txt` - Editor run stats

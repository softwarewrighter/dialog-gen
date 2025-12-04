# Dialog-Gen Project Status

## Current Status: **v0.1.0 - Functional**

Last Updated: 2024-12-04

## Completed

- [x] Project initialization (Cargo.toml, Rust 2024 edition)
- [x] Documentation structure
  - [x] architecture.md - System architecture overview
  - [x] prd.md - Product requirements document
  - [x] design.md - Technical design specification
  - [x] plan.md - Implementation plan
  - [x] status.md - This file
- [x] Demo data (tracked in git)
  - [x] demo/prompt1/ - Coffee shop AI debate
  - [x] demo/prompt2/ - VC pitch meeting
- [x] Core implementation
  - [x] src/error.rs - Custom error types
  - [x] src/config.rs - Input file parsing
  - [x] src/ollama.rs - Ollama HTTP client
  - [x] src/orchestrator.rs - Dialog generation loop
  - [x] src/output.rs - Output file writer
  - [x] src/main.rs - CLI integration
- [x] Testing with both demo scenarios
- [x] Code formatted (cargo fmt)
- [x] Clippy warnings resolved

## Not Started

- [ ] Unit tests
- [ ] Integration tests
- [ ] README.md with usage examples

## Development Environment

- **Rust Edition**: 2024
- **LLM Backend**: Ollama (local)
- **Default Model**: mistral:7b

## Quick Start

```bash
# Generate dialog from demo data
cargo run --release -- --input ./demo/prompt1 --verbose

# With custom model
cargo run --release -- --input ./demo/prompt1 --model llama3.2:latest

# Output to different directory
cargo run --release -- --input ./demo/prompt1 --output ./output
```

## CLI Options

```
Usage: dialog-gen [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>            Input directory containing configuration files
  -o, --output <OUTPUT>          Output directory (defaults to input directory)
  -m, --model <MODEL>            Ollama model to use [default: mistral:7b]
      --ollama-url <OLLAMA_URL>  Ollama server URL [default: http://localhost:11434]
  -v, --verbose                  Verbose output
  -h, --help                     Print help
  -V, --version                  Print version
```

## Demo Scenarios

| Directory | Scenario | Speakers |
|-----------|----------|----------|
| demo/prompt1/ | Coffee shop AI debate | Alex Chen vs Jordan Rivera |
| demo/prompt2/ | VC pitch meeting | Sarah Mitchell vs Michael Torres |

## Input File Format

Each input directory requires:
- `speaker1.txt` - First character definition
- `speaker2.txt` - Second character definition
- `directions.txt` - Scene context and director's notes
- `prompt.txt` - Initial dialog to seed conversation
- `scene.txt` - Configuration (turns, model, temperature)

## Known Issues

None currently identified.

## Metrics

| Metric | Value |
|--------|-------|
| Lines of Rust | ~600 |
| Test Coverage | 0% (no unit tests yet) |
| Documentation | 5 files |

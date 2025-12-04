# Dialog-Gen Architecture

## Overview

Dialog-Gen is a CLI tool that orchestrates conversations between two AI-driven characters using local LLM inference via Ollama. The system generates naturalistic dialog by alternating between two character perspectives, each with their own system prompts defining personality, motivation, and context.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Interface                            │
│                    (clap argument parsing)                       │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Input Directory Loader                      │
│  ┌───────────┬───────────┬────────────┬──────────┬───────────┐  │
│  │speaker1.txt│speaker2.txt│directions.txt│prompt.txt│scene.txt│  │
│  └───────────┴───────────┴────────────┴──────────┴───────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Dialog Orchestrator                           │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              System Prompt Builder                          ││
│  │  (combines speaker bio + directions for each character)     ││
│  └─────────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              Turn Manager                                   ││
│  │  (alternates between speakers for N iterations)             ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Ollama Client                               │
│              (HTTP requests to local Ollama server)              │
│                    Model: mistral:7b                             │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Output Writer                               │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  generated-dialog1.txt  │  generated-dialog2.txt  │  ...    ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. CLI Interface (`main.rs`)
- Parses command-line arguments using `clap`
- Primary argument: input directory path
- Optional: output directory, model override, verbosity

### 2. Config Loader (`config.rs`)
- Reads and validates all input files from the specified directory
- Structures:
  - `Speaker`: character name, bio, motivations
  - `Scene`: number of turns, any scene-specific settings
  - `Directions`: director's notes for the scene
  - `Prompt`: initial dialog/context

### 3. Dialog Orchestrator (`orchestrator.rs`)
- Core logic for managing the conversation flow
- Builds system prompts for each speaker
- Manages turn-taking between speakers
- Accumulates dialog history

### 4. Ollama Client (`ollama.rs`)
- HTTP client for Ollama API (`http://localhost:11434`)
- Handles `/api/generate` or `/api/chat` endpoints
- Manages streaming responses
- Error handling and retries

### 5. Output Writer (`output.rs`)
- Writes generated dialog to output files
- Formats output for TTS consumption
- One file per speaker for easy TTS processing

## Data Flow

1. **Initialization**
   - CLI parses input directory path
   - Config loader reads all `.txt` files
   - Validates required files exist

2. **System Prompt Construction**
   - Speaker 1 system prompt = speaker1.txt + directions.txt
   - Speaker 2 system prompt = speaker2.txt + directions.txt

3. **Dialog Generation Loop**
   ```
   conversation_history = parse_initial_prompt(prompt.txt)

   for turn in 0..scene.turns:
       if turn % 2 == 0:
           response = call_llm(speaker2_system, conversation_history)
           conversation_history.append(speaker2_response)
       else:
           response = call_llm(speaker1_system, conversation_history)
           conversation_history.append(speaker1_response)
   ```

4. **Output Generation**
   - Split final conversation by speaker
   - Write to separate files for TTS pipeline

## Directory Structure

```
dialog-gen/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── error.rs          # Custom error types
│   ├── config.rs         # Input file parsing
│   ├── orchestrator.rs   # Dialog generation logic
│   ├── ollama.rs         # LLM client
│   └── output.rs         # File output handling
├── docs/
│   ├── architecture.md   # This file
│   ├── prd.md            # Product requirements
│   ├── design.md         # Detailed design
│   ├── plan.md           # Implementation plan
│   └── status.md         # Project status
├── demo/                 # Sample input directories (tracked)
│   ├── prompt1/          # Coffee shop AI debate
│   └── prompt2/          # VC pitch meeting
└── test-data/            # Working test data (.gitignored)
```

## Technology Choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Performance, safety, existing crate structure |
| CLI Framework | clap | De facto standard for Rust CLIs |
| HTTP Client | reqwest | Async support, well-maintained |
| JSON | serde_json | Standard Rust JSON handling |
| Async Runtime | tokio | Required by reqwest |
| LLM Backend | Ollama (mistral:7b) | Local, free, good dialog quality |

## Future Considerations

- Support for multiple LLM backends (OpenAI, Anthropic)
- Configuration file for default settings
- Streaming output for real-time feedback
- Integration with TTS pipeline
- Support for more than 2 speakers

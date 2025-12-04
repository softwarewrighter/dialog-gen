# Dialog-Gen Implementation Plan

## Phase 1: Project Setup - COMPLETE

### Step 1.1: Update Cargo.toml
- [x] Edition: 2024
- [x] Add required dependencies:
  - clap (CLI parsing)
  - tokio (async runtime)
  - reqwest (HTTP client)
  - serde/serde_json (JSON handling)
  - thiserror (error types)
  - anyhow (error handling)

### Step 1.2: Create Module Structure
- [x] Create `src/config.rs`
- [x] Create `src/ollama.rs`
- [x] Create `src/orchestrator.rs`
- [x] Create `src/output.rs`
- [x] Create `src/error.rs`
- [x] Update `src/main.rs` with module declarations

## Phase 2: Core Implementation - COMPLETE

### Step 2.1: Error Types (`src/error.rs`)
- [x] Define `DialogGenError` enum
- [x] Implement error conversions
- [x] Create `Result` type alias

### Step 2.2: Configuration Loading (`src/config.rs`)
- [x] Define `Speaker` struct
- [x] Define `Scene` struct
- [x] Define `Directions` struct
- [x] Define `DialogConfig` struct
- [x] Implement file parsing for speaker files
- [x] Implement file parsing for directions.txt
- [x] Implement file parsing for scene.txt
- [x] Implement prompt.txt parsing
- [x] Add validation logic

### Step 2.3: Ollama Client (`src/ollama.rs`)
- [x] Define `OllamaClient` struct
- [x] Define `ChatMessage` struct
- [x] Implement `new()` constructor
- [x] Implement `health_check()` method
- [x] Implement `chat()` method

### Step 2.4: Dialog Orchestrator (`src/orchestrator.rs`)
- [x] Define `DialogOrchestrator` struct
- [x] Define `GeneratedDialog` and `DialogExchange` structs
- [x] Implement `build_system_prompt()` method
- [x] Implement conversation history building
- [x] Implement `generate()` method
- [x] Handle speaker alternation logic
- [x] Response cleaning and truncation

### Step 2.5: Output Writer (`src/output.rs`)
- [x] Define `OutputWriter` struct
- [x] Implement `next_file_number()` method
- [x] Implement `write()` method
- [x] Format output for TTS consumption

## Phase 3: CLI Integration - COMPLETE

### Step 3.1: Main Entry Point (`src/main.rs`)
- [x] Define CLI struct with clap derive
- [x] Implement argument parsing
- [x] Add input validation
- [x] Wire up all components
- [x] Add progress output (verbose mode)
- [x] Handle errors gracefully

### Step 3.2: Manual Testing
- [x] Test with demo/prompt1 (coffee shop debate)
- [x] Test with demo/prompt2 (VC pitch meeting)

## Phase 4: Polish - PARTIAL

### Step 4.1: Code Quality
- [x] cargo fmt
- [x] cargo clippy (no warnings)
- [ ] Unit tests
- [ ] Integration tests

### Step 4.2: Documentation
- [x] Architecture docs
- [x] Design docs
- [x] Status tracking
- [ ] README.md with usage examples

## Future Enhancements

- [ ] Streaming output for real-time feedback
- [ ] Support for multiple LLM backends (OpenAI, Anthropic)
- [ ] More than 2 speakers
- [ ] Direct TTS integration
- [ ] Configuration file for defaults

# Dialog-Gen Implementation Plan

## Phase 1: Project Setup

### Step 1.1: Update Cargo.toml
- [x] Edition: 2024 (current)
- [x] Add required dependencies:
  - clap (CLI parsing)
  - tokio (async runtime)
  - reqwest (HTTP client)
  - serde/serde_json (JSON handling)
  - thiserror (error types)
  - anyhow (error handling)

### Step 1.2: Create Module Structure
- [ ] Create `src/config.rs`
- [ ] Create `src/ollama.rs`
- [ ] Create `src/orchestrator.rs`
- [ ] Create `src/output.rs`
- [ ] Create `src/error.rs`
- [ ] Update `src/main.rs` with module declarations

## Phase 2: Core Implementation

### Step 2.1: Error Types (`src/error.rs`)
- [ ] Define `DialogGenError` enum
- [ ] Implement error conversions
- [ ] Create `Result` type alias

### Step 2.2: Configuration Loading (`src/config.rs`)
- [ ] Define `Speaker` struct
- [ ] Define `Scene` struct
- [ ] Define `Directions` struct
- [ ] Define `DialogConfig` struct
- [ ] Implement file parsing for speaker files
- [ ] Implement file parsing for directions.txt
- [ ] Implement file parsing for scene.txt
- [ ] Implement prompt.txt parsing
- [ ] Add validation logic
- [ ] Write unit tests

### Step 2.3: Ollama Client (`src/ollama.rs`)
- [ ] Define `OllamaClient` struct
- [ ] Define `ChatMessage` struct
- [ ] Implement `new()` constructor
- [ ] Implement `health_check()` method
- [ ] Implement `chat()` method
- [ ] Handle streaming responses
- [ ] Add timeout handling
- [ ] Write unit tests with mock server

### Step 2.4: Dialog Orchestrator (`src/orchestrator.rs`)
- [ ] Define `DialogOrchestrator` struct
- [ ] Define `GeneratedDialog` and `DialogExchange` structs
- [ ] Implement `build_system_prompt()` method
- [ ] Implement `parse_initial_prompt()` method
- [ ] Implement `generate()` method
- [ ] Handle speaker alternation logic
- [ ] Write unit tests

### Step 2.5: Output Writer (`src/output.rs`)
- [ ] Define `OutputWriter` struct
- [ ] Implement `next_file_number()` method
- [ ] Implement `write()` method
- [ ] Format output for TTS consumption
- [ ] Write unit tests

## Phase 3: CLI Integration

### Step 3.1: Main Entry Point (`src/main.rs`)
- [ ] Define CLI struct with clap derive
- [ ] Implement argument parsing
- [ ] Add input validation
- [ ] Wire up all components
- [ ] Add progress output
- [ ] Handle errors gracefully

### Step 3.2: Integration Testing
- [ ] Create integration test with test-data/prompt1
- [ ] Test error handling scenarios
- [ ] Test with different models

## Phase 4: Polish

### Step 4.1: User Experience
- [ ] Add colored output (optional)
- [ ] Improve error messages
- [ ] Add `--dry-run` option
- [ ] Add progress indicators

### Step 4.2: Documentation
- [ ] Update README.md
- [ ] Add usage examples
- [ ] Document input file formats

## Implementation Order

1. **Start with Ollama client** - validates LLM connectivity
2. **Configuration loading** - enables reading test data
3. **Orchestrator** - core logic depends on above
4. **Output writer** - simple, can be done in parallel
5. **CLI integration** - ties everything together
6. **Testing and polish** - final phase

## Estimated Module Sizes

| Module | Lines of Code |
|--------|---------------|
| error.rs | ~50 |
| config.rs | ~200 |
| ollama.rs | ~150 |
| orchestrator.rs | ~200 |
| output.rs | ~80 |
| main.rs | ~100 |
| **Total** | ~780 |

## Next Steps (Immediate)

1. Update `Cargo.toml` with dependencies
2. Create `src/error.rs` with error types
3. Create `src/ollama.rs` and test Ollama connectivity
4. Create `src/config.rs` to parse test-data files
5. Build out orchestrator and output modules
6. Integrate in main.rs

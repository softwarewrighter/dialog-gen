# dialog-gen

A Rust CLI tool that generates natural-sounding dialog between two AI-driven characters using local LLM inference via Ollama.

## Quick Start

```bash
# Install Ollama and pull a model
ollama pull llama3.2

# Run with demo input
cargo run --release -- -i ./demo/prompt3-llama -m llama3.2:latest --verbose
```

## Usage

```
dialog-gen [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    Input directory with config files
  -o, --output <OUTPUT>  Output directory (default: input dir)
  -m, --model <MODEL>    Ollama model [default: mistral:7b]
  -v, --verbose          Show generation progress
```

## Input Files

Each input directory needs:

| File | Purpose |
|------|---------|
| `speaker1.txt` | First character's name, background, personality, speaking style |
| `speaker2.txt` | Second character's definition |
| `directions.txt` | Scene context, setting, mood, director's notes |
| `prompt.txt` | Opening line(s) to seed the conversation |
| `scene.txt` | Number of turns, model, temperature |

## Output Files

- `generated-dialogN.txt` - The conversation
- `output-metadataN.txt` - Model, token counts, timing stats

---

## Demo Evolution

The `demo/` directory contains the evolution of prompt engineering and model selection to achieve natural-sounding dialog.

### Iteration 1: Verbose AI Debate

**[demo/prompt1/](demo/prompt1/)** - Two characters debating AI/automation

- **Problem**: Responses were multi-paragraph essays, not conversation
- **Characters**: Alex Chen (tech optimist) vs Jordan Rivera (skeptic journalist)
- **Output**: [generated-dialog1.txt](demo/prompt1/generated-dialog1.txt)

### Iteration 2: Verbose VC Pitch

**[demo/prompt2/](demo/prompt2/)** - Startup founder meets investor

- **Problem**: Still too verbose, felt like prepared statements
- **Characters**: Sarah Mitchell (founder) vs Michael Torres (VC)
- **Output**: [generated-dialog1.txt](demo/prompt2/generated-dialog1.txt)

### Iteration 3: Improved Prompting with mistral:7b

**[demo/prompt3-mistral/](demo/prompt3-mistral/)** - Podcast hosts discussing a fantasy book

- **Change**: Simplified system prompt, emphasized "ONE SHORT SENTENCE"
- **Characters**: Maya (enthusiastic co-host) vs Dev (analytical co-host)
- **Result**: Much shorter responses, but somewhat choppy/disconnected
- **Output**: [generated-dialog1.txt](demo/prompt3-mistral/generated-dialog1.txt)
- **Stats**: [output-metadata1.txt](demo/prompt3-mistral/output-metadata1.txt) (29.4 tokens/sec)

### Iteration 4: Better Model (llama3.2)

**[demo/prompt3-llama/](demo/prompt3-llama/)** - Same scenario, different model

- **Change**: Switched from mistral:7b to llama3.2:latest
- **Result**: Natural flowing conversation that builds on context
- **Output**: [generated-dialog1.txt](demo/prompt3-llama/generated-dialog1.txt)
- **Stats**: [output-metadata1.txt](demo/prompt3-llama/output-metadata1.txt) (54.3 tokens/sec)

### Key Learnings

1. **Simpler prompts work better** - Verbose instructions get ignored
2. **Model choice matters** - llama3.2 follows instructions better than mistral:7b
3. **Character voice comes from examples** - Speaking style descriptions help
4. **Director's notes set tone** - "CRITICAL" and "IMPORTANT" prefixes help

### Recommended Setup

For natural podcast-style dialog:
- Model: `llama3.2:latest`
- Temperature: `0.9`
- Turns: `10-15`
- System prompt: Keep it short, emphasize single-sentence responses

---

## Architecture

See [docs/architecture.md](docs/architecture.md) for system design.

## License

MIT

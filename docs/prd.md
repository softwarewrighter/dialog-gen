# Dialog-Gen Product Requirements Document

## Product Overview

Dialog-Gen is a command-line tool that generates realistic conversations between two AI-powered characters. It serves as the dialog generation component of a larger text-to-speech pipeline, enabling the creation of scripted conversations with distinct character voices and personalities.

## Problem Statement

Creating natural-sounding dialog between multiple characters is time-consuming and requires significant creative effort. Writers and content creators need a tool that can:
1. Generate contextually appropriate dialog
2. Maintain distinct character voices
3. Follow directorial guidance for scene tone and purpose
4. Produce output suitable for TTS processing

## Target Users

- **Content Creators**: Podcasters, video creators needing scripted conversations
- **Writers**: Authors exploring dialog options for their characters
- **Game Developers**: Creating NPC conversations and dialog trees
- **Educators**: Generating example conversations for language learning
- **Accessibility Teams**: Creating audio versions of written content

## Core Requirements

### Functional Requirements

#### FR1: Input Processing
- **FR1.1**: Accept a directory path containing input configuration files
- **FR1.2**: Read speaker definitions from `speaker1.txt` and `speaker2.txt`
- **FR1.3**: Read directorial guidance from `directions.txt`
- **FR1.4**: Read initial dialog/context from `prompt.txt`
- **FR1.5**: Read scene configuration from `scene.txt`

#### FR2: Character System Prompts
- **FR2.1**: Construct unique system prompts for each speaker
- **FR2.2**: Include character bio, personality, and motivations
- **FR2.3**: Include director's notes to guide tone and content
- **FR2.4**: Maintain character consistency throughout conversation

#### FR3: Dialog Generation
- **FR3.1**: Parse `prompt.txt` to extract initial context
- **FR3.2**: Alternate between speakers for specified number of turns
- **FR3.3**: Accumulate conversation history for context
- **FR3.4**: Generate responses that continue the conversation naturally

#### FR4: LLM Integration
- **FR4.1**: Connect to local Ollama server
- **FR4.2**: Use appropriate model (mistral:7b default)
- **FR4.3**: Handle API errors gracefully
- **FR4.4**: Support configurable model selection

#### FR5: Output Generation
- **FR5.1**: Write generated dialog to output directory
- **FR5.2**: Produce separate files per scene/run (generated-dialog1.txt, etc.)
- **FR5.3**: Format output for TTS consumption
- **FR5.4**: Include speaker labels for each line

### Non-Functional Requirements

#### NFR1: Performance
- Dialog generation should complete within reasonable time (dependent on LLM)
- Should not block on network issues indefinitely (timeouts)

#### NFR2: Usability
- Clear error messages for missing or malformed input files
- Progress indication during generation
- Helpful `--help` output

#### NFR3: Reliability
- Graceful handling of Ollama server unavailability
- Validation of input files before starting generation
- Recovery from transient network errors

#### NFR4: Maintainability
- Modular code structure for easy extension
- Clear separation of concerns
- Documentation of public APIs

## Input File Specifications

### speaker1.txt / speaker2.txt
```
Name: [Character Name]
Age: [Optional]
Background: [Character background and history]
Personality: [Key personality traits]
Motivations: [What drives this character]
Speaking Style: [How they talk - formal, casual, etc.]
```

### directions.txt
```
Scene: [Scene name/number]
Setting: [Where the conversation takes place]
Mood: [Emotional tone of the scene]
Goal: [What the scene should accomplish]
Notes: [Additional directorial guidance]
```

### prompt.txt
```
[Initial context or narration]

SPEAKER1: [Opening line of dialog]

[Optional additional lines to seed the conversation]
```

### scene.txt
```
turns: [Number of back-and-forth exchanges]
model: [Optional: LLM model override]
temperature: [Optional: creativity setting 0.0-1.0]
```

## Output File Specification

### generated-dialogN.txt
```
SPEAKER1: [Generated line]
SPEAKER2: [Generated response]
SPEAKER1: [Generated line]
...
```

## Success Metrics

1. **Functionality**: All input files processed correctly
2. **Quality**: Generated dialog maintains character voice
3. **Reliability**: Handles edge cases without crashing
4. **Usability**: Users can run the tool with minimal documentation

## Out of Scope (v1.0)

- GUI interface
- Real-time streaming output
- More than 2 speakers
- Direct TTS integration
- Cloud LLM support (OpenAI, Anthropic)
- Dialog editing/refinement workflows

## Future Enhancements

- Support for 3+ character conversations
- Integration with various LLM providers
- Built-in TTS pipeline
- Interactive mode for real-time dialog adjustment
- Character voice consistency scoring
- Export to screenplay format

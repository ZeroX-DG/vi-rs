# Project Agents.md Guide for AI agents

This Agents.md file provides comprehensive guidance for AI agents working with
this codebase.

## Project Structure

- `/src`: Source code that OpenAI Codex should analyze
- `/examples`: Examples of how to use the vi library. AI agent should maintain
  this if there's breaking change to library API
- `/tests`: Test files that OpenAI Codex should maintain and extend
- `/testdata`: Test data for snapshot testing defined in `/tests`
  - `/input`: Input test data for snapshot testing that the AI agent
  - `/output`: This is automatically generated when tests are run. AI agent
    should not attempt to change this.

## Coding Conventions

### General Conventions for Implementation

- Always format the project after applying changes by running `cargo fmt`.
- AI agent should follow the existing Rust langnuage code style in each file.
- Variable and function names should be meaningful and easy to read.
- AI agent should add comments to document functions. Especially public
  functions exposed in `lib.rs`.

## Testing Requirements

AI agent should run tests with the following commands:

```bash
cargo test
```

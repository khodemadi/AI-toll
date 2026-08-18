# aitool

A Rust toolkit for building AI agents with safe, extensible tools.

## Features

- Tool abstraction and registry
- Filesystem tool
- Shell tool
- Basic agent loop
- Provider abstraction
- CLI
- Async Rust with Tokio
- JSON tool input/output

## Build

```bash
cargo build
```

## Run

```bash
cargo run -p aitool-cli -- --help
cargo run -p aitool-cli -- tools
cargo run -p aitool-cli -- file-read Cargo.toml
cargo run -p aitool-cli -- shell "cargo --version"
```

## Architecture

```text
AI / Agent
    |
    v
Tool Registry
    |
    +--> filesystem
    +--> shell
    +--> web        (future)
    +--> git        (future)
    |
    v
Tool Result
```

The project is intentionally split into crates so providers, tools, and the agent can evolve independently.

## Roadmap

- [ ] OpenAI-compatible provider
- [ ] Ollama provider
- [ ] MCP server
- [ ] Web search/fetch tools
- [ ] Git tools
- [ ] Permissions and sandboxing
- [ ] Persistent memory
- [ ] Streaming responses
- [ ] Tool-call JSON schema


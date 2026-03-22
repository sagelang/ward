<p align="center">
  <img src="https://raw.githubusercontent.com/sagelang/sage/main/assets/ward.png" alt="Ward the Owl" width="200">
</p>

<h1 align="center">Ward</h1>

<p align="center">
  <strong>An interactive coding agent built with Sage.</strong><br>
  <em>The owl is watching.</em>
</p>

<p align="center">
  <a href="https://github.com/sagelang/sage">Sage Language</a> •
  <a href="https://sagelang.github.io/sage-book">Guide</a> •
  <a href="#usage">Usage</a>
</p>

---

Ward is a terminal-based coding assistant — a reference application for the [Sage programming language](https://github.com/sagelang/sage). Type naturally, and Ward will answer questions, run shell commands, and create files.

```
  ┌──────────────────────────────────────────┐
  │                                          │
  │            ___                           │
  │           (o,o)                          │
  │           {`"'`}                         │
  │           -"-"-                          │
  │                                          │
  │    W A R D  The Sage Coding Agent        │
  │    v0.1.0                                │
  │                                          │
  └──────────────────────────────────────────┘
```

## Features

- **Natural language input** — just type what you want, no command prefixes needed
- **Conversation memory** — Ward remembers context within a session
- **Shell execution** — the LLM can run commands and use the output in its response
- **File creation** — generates and saves files to disk, auto-opens HTML in the browser
- **Sage agent architecture** — supervision trees, persistent state, extern Rust FFI

## Usage

### Prerequisites

- [Sage](https://github.com/sagelang/sage) v2.0.0+
- An OpenAI-compatible API key for LLM features

### Run

```bash
export SAGE_API_KEY="your-api-key"
cd ward
sage run .
```

### Commands

| Command | Description |
|---------|-------------|
| `help` | Show available commands |
| `init` | Initialize Ward in current project |
| `scan` | Scan project structure |
| `status` | Show session status |
| `clear` | Clear the screen |
| `exit` | Leave Ward |

Anything else is treated as natural language and sent to the LLM.

### Examples

```
▸ create an html page that says "hello world" and open it
▸ is anything running on port 3000?
▸ list all .rs files in this directory
▸ write a python script that converts csv to json
```

## Architecture

Ward is written in ~120 lines of Sage with ~280 lines of Rust extern functions for terminal I/O, styling, and file operations.

```
ward/
├── grove.toml           # Sage project manifest
├── src/
│   ├── main.sg          # Agent logic (REPL, commands, LLM integration)
│   └── sage_extern.rs   # Rust FFI (terminal, styling, file actions)
```

**Agent model:** Ward runs as a single agent under a `OneForOne` supervisor. The agent's `on start` handler runs the REPL loop, and `@persistent` fields track session state.

**Tool use:** The LLM is instructed to output `<<<SHELL:command>>>` markers for shell execution and `<<<SAVE:filename>>>...<<<END>>>` markers for file creation. Ward parses these, executes the actions, and feeds results back to the LLM for a final response.

## Related

- [sagelang/sage](https://github.com/sagelang/sage) — The Sage programming language
- [sagelang/rfcs](https://github.com/sagelang/rfcs) — Language design RFCs
- [sagelang/sage-book](https://github.com/sagelang/sage-book) — Documentation

## License

MIT

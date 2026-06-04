<p align="center">
  <img src="https://raw.githubusercontent.com/sagelang/sage/main/assets/ward.png" alt="Ward the Owl" width="200">
</p>

<h1 align="center">Ward</h1>

<p align="center">
  <strong>An agentic coding agent built in Sage.</strong><br>
  <em>The owl is watching.</em>
</p>

<p align="center">
  <a href="https://github.com/sagelang/sage">Sage Language</a> •
  <a href="https://sagelang.github.io/sage-book">Guide</a> •
  <a href="#usage">Usage</a> •
  <a href="#architecture">Architecture</a>
</p>

---

Ward is a terminal coding agent — the reference application for the [Sage programming language](https://github.com/sagelang/sage). Give it a task in plain English and it will read files, edit them surgically, run commands, observe the results, and **keep going** until the task is done — asking permission before it touches anything.

It is a genuine **act → observe → act** loop, not a single-shot assistant. The loop *is* `agent` + `divine` + tool dispatch; the hands *are* the `Fs` and `Shell` tools; crash-safety *is* a supervision tree. An owl, written in Sage, that helps you write Sage. See [RFC-0024](https://github.com/sagelang/rfcs) for the design of record.

```
  ┌──────────────────────────────────────────┐
  │                                          │
  │            ___                           │
  │           (o,o)                          │
  │           {`"'`}                         │
  │           -"-"-                          │
  │                                          │
  │    W A R D  The Sage Coding Agent        │
  │    v0.3.0                                │
  │                                          │
  └──────────────────────────────────────────┘
```

## Features

- **Agentic loop** — perceive → decide → act → observe, repeated until the task is done (hard step cap to bound runaway loops).
- **A real tool layer** — `read`, `list`, `search`, `bash`, `write`, `edit`. The minimum set that makes an agent useful on a codebase.
- **Surgical edits** — read-before-edit, unique-match replacement with a printed red/green diff. Never a blind whole-file clobber.
- **Permission by default** — every side-effecting action (`edit`/`write`/`bash`) shows a preview and asks `[y/N]`. Opt into `auto` mode for trusted, fast iteration.
- **Structured memory** — a typed transcript of actions and observations, truncated per-entry to protect the context window.
- **Pure Sage** — no Rust FFI, no SDK. `grove.toml` is clean: the entire I/O surface is the `Fs` and `Shell` tools.
- **Crash-safe** — runs as a `Transient` child under a `OneForOne` supervisor; a mid-task crash restarts the agent, not the process. Session state is `@persistent`.

## Usage

### Prerequisites

- [Sage](https://github.com/sagelang/sage) v2.2.0+
- An OpenAI-compatible API key for the LLM (`divine`)

### Run

```bash
export SAGE_API_KEY="your-api-key"
cd ward
sage run .
```

> **Note on this checkout.** Two toolchain issues currently sit between `sage run .` and a running binary, both independent of Ward's own code:
> 1. The 2.2.0 CLI generates a build that depends on `sage-runtime 2.2.0`, which isn't published on crates.io yet.
> 2. The cross-module supervisor fix Ward needs lives in a patched compiler (see [`docs/sage-notes.md`](docs/sage-notes.md)).
>
> The [`run-ward.sh`](run-ward.sh) harness works around both — it builds against the local Sage source and prefers the patched compiler if present. See the dev notes for details.

### Commands

| Command        | Description |
|----------------|-------------|
| `help`         | Show available commands |
| `init`         | Initialize Ward in the current project |
| `scan`         | Scan project structure |
| `status`       | Show session status (sessions, cwd, auto mode, time) |
| `auto on`/`off`| Toggle auto-approve (skip the `[y/N]` gate) |
| `clear`        | Clear the screen |
| `exit`         | Leave Ward |

Anything else is treated as a task and handed to the agentic loop.

### Examples

```
▸ add a --verbose flag to the CLI and wire it through to the logger
▸ find every TODO in the codebase and turn them into a checklist file
▸ the tests in foo_test.sg are failing — read them, fix the bug, re-run
▸ create an html page that says "hello world" and open it
```

## Architecture

Ward is written in pure Sage, split into focused modules. Pure logic (formatting,
decoding, diffing) lives in tool-free modules; the `Fs`/`Shell`-touching executors
live in the agent, where Sage requires tool use to be `use`-declared.

```
ward/
├── grove.toml          # Sage project manifest
├── run-ward.sh         # dev runtime harness (see docs/sage-notes.md)
├── docs/
│   └── sage-notes.md   # language/compiler issues found + fixes
└── src/
    ├── main.sg         # module root: mod declarations, supervisor, run
    ├── core.sg         # the Ward agent + act-observe loop + Fs/Shell tools
    ├── proto.sg        # tag-protocol decoding (pure)
    ├── tools.sg        # observation formatting: line numbers, truncation (pure)
    ├── ui.sg           # palette, banner, status chrome, diff, confirm (pure)
    └── config.sg       # application constants (pure)
```

**The loop.** Each turn, Ward builds a prompt from the transcript, asks the model
(`divine`) for exactly **one** action, executes it against the tools, records a
structured observation, and repeats — until the model emits `<finish>` or the
step cap is hit.

**The decision channel.** v0.3.0 ships a one-action-per-reply *tag protocol*
(`<read>`, `<edit>`, `<bash>`, …). Decoding is pure string work in `proto.sg`.
The target end-state is typed `Oracle<ToolAction>` structured output behind the
same dispatch — a local change to one module.

**The `edit` tool** is the load-bearing primitive: it confirms the file exists,
counts occurrences of the `<old>` text (0 → "not found, re-read"; >1 → "ambiguous,
add context"), and only on a unique match renders a diff, gates on `[y/N]`, and
writes. Line-numbered `read` output is what makes that contract reliable.

## Related

- [sagelang/sage](https://github.com/sagelang/sage) — The Sage programming language
- [sagelang/rfcs](https://github.com/sagelang/rfcs) — Language design RFCs (Ward is RFC-0024)
- [sagelang/sage-book](https://github.com/sagelang/sage-book) — Documentation

## License

MIT

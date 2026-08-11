# Sage notes — issues found building Ward v0.3.0

Splitting Ward into modules and landing the v0.3.0 agentic loop surfaced eight
concrete Sage issues. Six were worked around in Ward's own source; two were
genuine compiler bugs fixed in `sage-codegen`. This is the record of each — both
as documentation for Ward and as a starting point for upstreaming.

The Sage source referenced below is the Homebrew tap checkout:
`/opt/homebrew/Library/Taps/cargopete/homebrew-sage`.

---

## Worked around in Ward

### 1. Reserved keywords can't be module names — and the failure is silent

`protocol` and `agent` (and `tool`, `handler`, `start`, …) are reserved keywords.
Naming a module after one — `mod protocol;`, `mod agent;` — does not parse as a
module declaration. Worse: the loader **silently swallows** the resulting parse
error *and every `mod` declaration after it in the file*, then reports success.
`sage check .` printed "no errors" while three modules were never loaded or
type-checked at all (a deliberate type error inside an unloaded module also went
unreported).

- **Diagnosis:** `sage check src/<file>.sg` (checking a single file directly)
  *does* surface the parse error; only the project loader swallows it.
- **Fix in Ward:** renamed the modules to `proto` and `core`.
- **Suggested upstream fix:** the loader should propagate per-module parse
  errors instead of dropping the module; `mod <keyword>;` should be a hard error.

**It is not only module names.** A reserved word used as a *parameter* name fails
the same way and is harder to spot. `pub fn number_lines_from(s: String, start: Int)`
kills the parse at that line, and the rest of the file goes with it: `truncate`,
`shell_quote` and `count_occurrences`, all defined lower down and untouched, simply
ceased to exist. The reported error was `item 'truncate' not found in module 'tools'`
with a span pointing into a comment in a different file. Nothing mentioned `start`.

Worth knowing when a module half-disappears: the failure is at the first reserved
identifier, and everything below it in the file is collateral.

The full reserved list (Sage 2.2.0, from `sage-parser/src/token.rs`):

```
agent as await belief break catch checkpoint children const delay divine else
enum error extern fail fails false fn follows for handler handles if in infer
let loop match message mock mod on pause protocol pub receive receives record
reply restart resting resume retry return run self send span start stop strategy
summon super supervisor test timeout tool trace true try use waking while yield
```

### 2. `read_line` forces its function `fails`, and a `fails`→`Bool` fn mis-compiles

`read_line()` makes its enclosing free function fallible even when wrapped in a
total `try … catch`. Declaring the function `fails` satisfies the checker, but
codegen then emits broken Rust for a `fails` function that returns a `Bool`: the
`return <bool-expr>;` isn't `Ok(...)`-wrapped, and the `try confirm(...) catch`
call site lowers to an invalid `match`.

- **Fix in Ward:** dropped the single `confirm()` helper; split it into two
  *pure* helpers — `confirm_prompt(question) -> String` and `is_yes(answer) -> Bool`
  — with the `read_line` done inline in the agent handler (a fallible context
  that already has `on error`). See `ui.sg` and `core.sg`.

### 3. `divine` only accepts a string *literal* template

`divine` is a keyword, not a function: it parses `divine("template" [-> Type])`
where the template must be a string literal (with `{interpolation}`). You cannot
build a prompt in a helper and pass the `String` in (`divine(prompt)` →
`parse error: expected "<string>"`).

- **Fix in Ward:** inlined the protocol prompt at the `divine` call site in
  `core.sg` with `{env_block}`/`{transcript}` interpolation. `proto.sg` documents why
  `build_prompt` can't exist as a function today.
- **Suggested upstream fix:** allow `divine(<expr: String>)` so the prompt can be
  assembled separately (would also make the prompt pure and unit-testable).

### 4. `str(str_len(x))` generates invalid Rust

`str_len(x)` lowers to `x.chars().count() as i64`; feeding that straight into
`str(...)` (`.to_string()`) produces `... as i64.to_string()` — Rust rejects a
method call directly on a cast (`error: cast cannot be followed by a method
call`). The cast needs wrapping parens that codegen doesn't emit.

- **Fix in Ward:** bind the length to a variable first (`let n = str_len(x); str(n)`).
- **Suggested upstream fix:** wrap cast expressions in parens when they are the
  receiver of a method call.

### 5. `sage build` with no argument builds in single-file mode

`sage build` defaults its `file` argument to the `entry` in `grove.toml`, and a
file argument means single-file mode. So the default invocation of a multi-module
project cannot resolve its own modules:

```
× item `Ward` not found in module `core`
  use core::Ward;
```

The banner is the tell: "compiling main.sg" is single-file mode, "compiling
project" is what you want.

- **Fix in Ward:** `sage build . --emit-rust` in `scripts/build.sh`, with a comment
  saying why the `.` is load-bearing.
- **Suggested upstream fix:** when a `grove.toml` is present, default to project
  mode. Falling back to single-file mode for the manifest's own entry point is
  never the intent.

### 6. A Sage variable named `ctx` breaks `divine`

`divine` lowers to `ctx.infer_string(&format!(...))`, where `ctx` is the
`AgentContext` parameter of the generated handler. The name is not hygienic, so a
Sage local called `ctx` shadows it and the call lands on the wrong receiver:

```
error[E0599]: no method named `infer_string` found for struct `String`
```

Found while adding the project-context block: `let ctx = dup(context);` next to a
`divine` call, which is the most natural name imaginable for that variable.

- **Fix in Ward:** renamed the local to `env_block`.
- **Suggested upstream fix:** emit the context parameter under a reserved name
  (`__sage_ctx`), or reject `ctx` as an identifier. Silently shadowing the runtime
  handle produces an error message that points nowhere near the cause.

The same class of problem covers *Rust* keywords that Sage happily accepts. A
`let where = ...` in `core.sg` lowers to `let where` and the generated crate fails
with `expected identifier, found keyword 'where'`. Sage identifiers should be
escaped on the way out (`r#where`), since the set of Rust keywords is not something
a Sage author should have to carry in their head.

Watch for moves as well: a `String` bound once and used three times in a list
literal produces `use of moved value`, because codegen emits the binding rather
than a clone. Call the producing function again, or `dup()`.

---

## Fixed in the compiler (`sage-codegen/src/generator.rs`)

Both are cross-module codegen bugs: state collected per-module (or only from the
root) instead of across the whole module tree. Both fixes add a global pre-pass.

### 7. Supervisor can't construct an agent declared in another module

`generate_supervisor_main` looked up its child agent only in the **root module's**
`program.agents`. With `agent Ward` in `core.sg` and the supervisor in `main.sg`,
the lookup returned `None`, so codegen emitted a fieldless `let agent = Ward;`
even though `struct Ward { … }` has fields → `error[E0423]: expected value, found
struct Ward`.

- **Fix:** added an `all_agents: HashMap<String, AgentDecl>` registry populated in
  the all-modules pre-pass (`collect_agent_metadata`), used as a fallback in the
  child lookup. Now the supervisor emits the full `Ward { … }` constructor with
  all `@persistent` initializers.

### 8. Cross-module `String` consts are `.clone()`'d, not `.to_string()`'d

A `String` const is emitted as `&'static str`; referencing it where a `String` is
expected requires `.to_string()`. Codegen tracked which consts are strings only
as it *emitted* each module, so a reference compiled **before** the defining
module had been emitted (module emission order is a non-deterministic `HashMap`
walk) got a bare `.clone()` and stayed `&str` → `error[E0308]: expected String,
found &str` (e.g. `banner(VERSION)` with `VERSION` in `config.sg`).

- **Fix:** collect all `String` consts across every module in the pre-pass, so the
  classification is order-independent.

### Building / using the patched compiler

```bash
cd /opt/homebrew/Library/Taps/cargopete/homebrew-sage
cargo build --manifest-path crates/sage-cli/Cargo.toml --bin sage
# → target/debug/sage   (scripts/build.sh prefers this automatically)
```

The patches are self-contained additions (a struct field, two pre-pass loops, one
fallback lookup); `cargo install` would make them the default `sage` if desired.

---

## Bonus: running at all (`sage run` in this environment)

Unrelated to Ward's code: `sage run`/`build` generates a hearth project that
depends on `sage-runtime = "^2.2.0"`, which isn't published on crates.io (latest
published is 2.1.0), so cargo resolution fails *after* codegen. `scripts/build.sh`
generates the hearth Rust with `--emit-rust`, patches its `Cargo.toml` to a local
Sage checkout via `[patch.crates-io]`, and builds with cargo directly. Set
`SAGE_SRC=""` to skip the patch once 2.2.0 publishes.

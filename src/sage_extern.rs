//! Ward extern functions — Rust implementations called from Sage via extern fn.
//!
//! Ward's visual identity uses the official Sage mascot palette:
//!   Feather Base Dark  #3D4F44  rgb(61, 79, 68)
//!   Feather Mid Sage   #5A7A62  rgb(90, 122, 98)
//!   Feather Light      #8FA88F  rgb(143, 168, 143)
//!   Brow/Accent Gold   #D4A017  rgb(212, 160, 23)
//!   Gold Highlights    #F0C040  rgb(240, 192, 64)
//!   Background Dark    #1A1F1E  rgb(26, 31, 30)
//!
//! NOTE: All functions take `String` (not `&str`) because Sage codegen passes
//! owned strings to extern calls.

use std::io::{self, Write};

// =============================================================================
// Ward's colour palette — 24-bit ANSI escape codes
// =============================================================================

const SAGE_DARK: &str = "\x1b[38;2;61;79;68m";
const SAGE_MID: &str = "\x1b[38;2;90;122;98m";
const SAGE_LIGHT: &str = "\x1b[38;2;143;168;143m";
const GOLD: &str = "\x1b[38;2;212;160;23m";
const GOLD_BRIGHT: &str = "\x1b[38;2;240;192;64m";
#[allow(dead_code)]
const BG_DARK: &str = "\x1b[48;2;26;31;30m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[38;2;180;60;50m";
const WARM_WHITE: &str = "\x1b[38;2;220;215;200m";

// =============================================================================
// Process primitives
// =============================================================================

/// Get command-line arguments (excluding program name).
pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

/// Get a single CLI argument by index, or empty string if absent.
pub fn arg_at(index: i64) -> String {
    std::env::args()
        .skip(1)
        .nth(index as usize)
        .unwrap_or_default()
}

/// Join CLI arguments from a given index onward into a single string.
pub fn join_args(from: i64) -> String {
    std::env::args()
        .skip(1 + from as usize)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Read an environment variable.
pub fn env_var(key: String) -> Option<String> {
    std::env::var(&key).ok()
}

/// Exit the process.
pub fn exit_process(code: i64) {
    std::process::exit(code as i32)
}

/// Get current working directory.
pub fn cwd() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".".to_string())
}

/// Get current time as ISO 8601 string.
pub fn now_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// =============================================================================
// Terminal I/O
// =============================================================================

/// Print a prompt message and read a line from stdin.
/// Returns "quit" on EOF (Ctrl-D) so the REPL exits cleanly.
pub fn prompt(msg: String) -> String {
    print!("{GOLD}{BOLD}{msg}{RESET} ");
    io::stdout().flush().ok();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => return "quit".to_string(),
        Ok(_) => {}
    }
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    input
}

// =============================================================================
// Ward's visual language
// =============================================================================

/// Style text using Ward's palette.
pub fn styled(text: String, style: String) -> String {
    match style.as_str() {
        "gold" => format!("{GOLD}{text}{RESET}"),
        "bright" => format!("{GOLD_BRIGHT}{text}{RESET}"),
        "sage" => format!("{SAGE_MID}{text}{RESET}"),
        "sage_light" => format!("{SAGE_LIGHT}{text}{RESET}"),
        "sage_dark" => format!("{SAGE_DARK}{text}{RESET}"),
        "warm" => format!("{WARM_WHITE}{text}{RESET}"),
        "dim" => format!("{DIM}{SAGE_LIGHT}{text}{RESET}"),
        "bold" => format!("{BOLD}{WARM_WHITE}{text}{RESET}"),
        "bold_gold" => format!("{BOLD}{GOLD}{text}{RESET}"),
        "bold_bright" => format!("{BOLD}{GOLD_BRIGHT}{text}{RESET}"),
        "error" => format!("{BOLD}{RED}{text}{RESET}"),
        _ => text,
    }
}

/// Horizontal rule in sage green.
pub fn hr(width: i64) -> String {
    let line = "\u{2500}".repeat(width as usize);
    format!("{SAGE_DARK}{line}{RESET}")
}

/// Section header — gold text with sage-green box drawing.
pub fn header(title: String) -> String {
    let bar = "\u{2500}".repeat(3);
    format!(
        "\n{SAGE_MID}\u{256D}{bar}\u{2500}{RESET} {BOLD}{GOLD}{title}{RESET} {SAGE_MID}\u{2500}{bar}\u{256E}{RESET}\n"
    )
}

/// Status line: steward name in gold, message in sage.
pub fn status(steward: String, msg: String) -> String {
    format!(
        "  {SAGE_DARK}\u{2502}{RESET} {GOLD}{steward}{RESET} {SAGE_LIGHT}{msg}{RESET}"
    )
}

/// Success indicator — gold checkmark.
pub fn success(msg: String) -> String {
    format!(
        "  {SAGE_DARK}\u{2502}{RESET} {GOLD_BRIGHT}\u{2713}{RESET} {WARM_WHITE}{msg}{RESET}"
    )
}

/// Failure indicator — red X.
pub fn failure(msg: String) -> String {
    format!(
        "  {SAGE_DARK}\u{2502}{RESET} {RED}\u{2717}{RESET} {WARM_WHITE}{msg}{RESET}"
    )
}

/// Warning indicator.
pub fn warning(msg: String) -> String {
    format!(
        "  {SAGE_DARK}\u{2502}{RESET} {GOLD}\u{25B2}{RESET} {WARM_WHITE}{msg}{RESET}"
    )
}

/// Ward's banner — the owl ASCII art in gold, with version.
pub fn banner(version: &str) -> String {
    format!(
        r#"
{SAGE_DARK}  ┌──────────────────────────────────────────┐{RESET}
{SAGE_DARK}  │{RESET}                                          {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {GOLD}          ___          {RESET}               {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {GOLD_BRIGHT}         (o,o)         {RESET}               {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {SAGE_MID}         {{`"'`}}         {RESET}               {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {SAGE_DARK}         -"-"-         {RESET}               {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}                                          {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {BOLD}{GOLD}  W A R D{RESET}  {SAGE_LIGHT}The Sage Coding Agent{RESET}   {SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}  {SAGE_DARK}  v{version}                              {RESET}{SAGE_DARK}│{RESET}
{SAGE_DARK}  │{RESET}                                          {SAGE_DARK}│{RESET}
{SAGE_DARK}  └──────────────────────────────────────────┘{RESET}
"#
    )
}

/// Footer line to close a section.
pub fn footer() -> String {
    let bar = "\u{2500}".repeat(3);
    format!("{SAGE_MID}\u{2570}{bar}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}{RESET}\n")
}

/// Phase header — marks the start of a workflow phase.
pub fn phase(number: i64, title: String) -> String {
    format!(
        "\n  {GOLD_BRIGHT}\u{25C6}{RESET} {BOLD}{GOLD}Phase {number}{RESET} {SAGE_LIGHT}\u{2500} {title}{RESET}\n"
    )
}

/// Indent a block of text for display inside a section.
pub fn indent(text: String, prefix: String) -> String {
    text.lines()
        .map(|line| format!("  {SAGE_DARK}\u{2502}{RESET} {prefix}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Ward speaks — a terse, authoritative message.
pub fn ward_says(msg: String) -> String {
    format!(
        "  {SAGE_DARK}\u{2502}{RESET} {GOLD}\u{25B8}{RESET} {BOLD}{WARM_WHITE}Ward{RESET}{SAGE_LIGHT}: {msg}{RESET}"
    )
}

/// Clear the terminal screen.
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();
}

// =============================================================================
// File actions — parse LLM output and save files
// =============================================================================

/// Parse LLM response for <<<SAVE:filename>>>...<<<END>>> blocks.
/// Saves each file, opens HTML files in the browser, and returns styled status.
/// Returns empty string if no files were found.
pub fn save_files_from_response(response: String) -> String {
    let mut messages = Vec::new();
    let mut remaining = response.as_str();

    while let Some(start_idx) = remaining.find("<<<SAVE:") {
        let after_marker = &remaining[start_idx + 8..];
        let Some(name_end) = after_marker.find(">>>") else { break };
        let filename = after_marker[..name_end].trim();
        let after_name = &after_marker[name_end + 3..];
        let Some(end_idx) = after_name.find("<<<END>>>") else { break };
        let content = after_name[..end_idx]
            .trim_start_matches('\n')
            .trim_end_matches('\n');

        // Create parent directories if needed
        if let Some(parent) = std::path::Path::new(filename).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).ok();
            }
        }

        match std::fs::write(filename, content) {
            Ok(_) => {
                messages.push(success(format!("Saved {filename}")));
                if filename.ends_with(".html") || filename.ends_with(".htm") {
                    std::process::Command::new("open").arg(filename).spawn().ok();
                    messages.push(status("      ".to_string(), "Opened in browser".to_string()));
                }
            }
            Err(e) => {
                messages.push(failure(format!("Failed to save {filename}: {e}")));
            }
        }

        remaining = &after_name[end_idx + 9..];
    }

    messages.join("\n")
}

/// Remove all <<<SAVE:...>>>...<<<END>>> and <<<SHELL:...>>> markers for display.
pub fn clean_response(response: String) -> String {
    let mut text = response;

    // Strip <<<SAVE:filename>>>...<<<END>>> blocks
    loop {
        let Some(start) = text.find("<<<SAVE:") else { break };
        let Some(end_rel) = text[start..].find("<<<END>>>") else { break };
        let end = start + end_rel + 9;
        text = format!("{}{}", &text[..start], &text[end..]);
    }

    // Strip <<<SHELL:command>>> markers
    loop {
        let Some(start) = text.find("<<<SHELL:") else { break };
        let Some(end_rel) = text[start + 9..].find(">>>") else { break };
        let end = start + 9 + end_rel + 3;
        text = format!("{}{}", &text[..start], &text[end..]);
    }

    text.trim().to_string()
}

/// Extract the command from the first <<<SHELL:command>>> marker, or empty string.
pub fn extract_shell_command(response: String) -> String {
    if let Some(start) = response.find("<<<SHELL:") {
        let after = &response[start + 9..];
        if let Some(end) = after.find(">>>") {
            return after[..end].trim().to_string();
        }
    }
    String::new()
}

/// Run a shell command and return stdout+stderr. Infallible — errors become strings.
pub fn safe_shell_run(cmd: String) -> String {
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let mut result = String::new();
            if !stdout.is_empty() {
                result.push_str(&stdout);
            }
            if !stderr.is_empty() {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(&stderr);
            }
            if result.is_empty() {
                "(no output)".to_string()
            } else {
                result
            }
        }
        Err(e) => format!("Error running command: {e}"),
    }
}

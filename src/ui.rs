// zippy UI — Zeta color palette terminal rendering
// Palette: bg=#0D1117, surface=#151b23, text=#D8DEE9, muted=#ADB5BD, accent=#EAB308

#![allow(dead_code)]
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Zeta brand colors as ANSI 24-bit escape sequences
pub struct Zeta;

impl Zeta {
    pub const BG: &'static str = "\x1b[48;2;13;17;23m";
    pub const SURFACE: &'static str = "\x1b[48;2;21;27;35m";
    pub const TEXT: &'static str = "\x1b[38;2;216;222;233m";
    pub const MUTED: &'static str = "\x1b[38;2;173;181;189m";
    pub const ACCENT: &'static str = "\x1b[38;2;234;179;8m";
    pub const ACCENT_H: &'static str = "\x1b[38;2;250;197;21m";
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
}

// ─── Logo ───────────────────────────────────────────────────────────────────

pub fn print_logo() {
    let a = Zeta::ACCENT;
    let b = Zeta::BOLD;
    let m = Zeta::MUTED;
    let r = Zeta::RESET;
    println!();
    println!("{a}╔════════════════════════════════════╗{r}");
    println!("{a}║{r}                                    {a}║{r}");
    println!("{a}║{r}  {b}███████╗███████╗████████╗{b}{r}  {a}║{r}");
    println!("{a}║{r}  {b}╚══███╔╝╚══███╔╝╚══███╔╝{b}{r}  {a}║{r}");
    println!("{a}║{r}  {b}  ███╔╝   ███╔╝   ███╔╝{b}{r}   {a}║{r}");
    println!("{a}║{r}  {b} ███╔╝   ███╔╝   ███╔╝{b}{r}    {a}║{r}");
    println!("{a}║{r}  {b}███████╗███████╗███████╗{b}{r}  {a}║{r}");
    println!("{a}║{r}  {b}╚══════╝╚══════╝╚══════╝{b}{r}  {a}║{r}");
    println!("{a}║{r}                                    {a}║{r}");
    println!("{a}║{r}  {m}Zeta Toolchain Installer{r}        {a}║{r}");
    println!("{a}║{r}  {m}v0.1.0 — z-lang.org{r}           {a}║{r}");
    println!("{a}╚════════════════════════════════════╝{r}");
    println!();
}

// ─── Status Box ─────────────────────────────────────────────────────────────

pub fn print_success_box(lines: &[&str]) {
    let a = Zeta::ACCENT;
    let t = Zeta::TEXT;
    let r = Zeta::RESET;
    let width = 40;
    let top = format!("{a}╔{}╗{r}", "═".repeat(width));
    let bottom = format!("{a}╚{}╝{r}", "═".repeat(width));
    println!();
    println!("{top}");
    for line in lines {
        let line = &format!("  {t}{}{r}", line);
        let padding = if line.chars().count() <= width { width - line.len() } else { 0 };
        println!("{a}║{r}{}{}{a}║{r}", line, " ".repeat(padding));
    }
    println!("{bottom}");
    println!();
}

// ─── Step Printer ───────────────────────────────────────────────────────────

pub fn step(msg: &str) {
    println!(
        "  {bold}{accent}▶{reset} {text}{}{reset}",
        msg,
        bold = Zeta::BOLD,
        accent = Zeta::ACCENT,
        text = Zeta::TEXT,
        reset = Zeta::RESET,
    );
}

pub fn step_ok(msg: &str) {
    println!(
        "  {bold}{muted}└─{reset} {green}✓{reset} {muted}{}{reset}",
        msg,
        bold = Zeta::BOLD,
        muted = Zeta::MUTED,
        green = "\x1b[38;2;34;197;94m",
        reset = Zeta::RESET,
    );
}

pub fn step_fail(msg: &str) {
    eprintln!(
        "  {bold}{accent}▶{reset} {red}✗{reset} {text}{}{reset}",
        msg,
        bold = Zeta::BOLD,
        accent = Zeta::ACCENT,
        red = "\x1b[38;2;239;68;68m",
        text = Zeta::TEXT,
        reset = Zeta::RESET,
    );
}

// ─── Progress Bar ───────────────────────────────────────────────────────────

pub fn make_progress(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "  {bold}{accent}▶{reset} {msg} {wide_bar} {bytes}/{total_bytes}",
            )
            .unwrap()
            .progress_chars("━━━")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
    );
    pb
}

// ─── Spinner ────────────────────────────────────────────────────────────────

pub fn make_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.yellow} {bold}{accent}▶{reset} {msg}")
            .unwrap()
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

// ─── Error ───────────────────────────────────────────────────────────────────

pub fn error(msg: &str) {
    eprintln!();
    eprintln!(
        "  {red}╔══════════════════════════════╗{reset}",
        red = "\x1b[38;2;239;68;68m",
        reset = Zeta::RESET,
    );
    eprintln!(
        "  {red}║{reset}  {bold}{red}✗ Error{reset}                   {red}║{reset}",
        red = "\x1b[38;2;239;68;68m",
        bold = Zeta::BOLD,
        reset = Zeta::RESET,
    );
    eprintln!(
        "  {red}║{reset}  {text}{}{reset}            {red}║{reset}",
        msg,
        red = "\x1b[38;2;239;68;68m",
        text = Zeta::TEXT,
        reset = Zeta::RESET,
    );
    eprintln!(
        "  {red}╚══════════════════════════════╝{reset}",
        red = "\x1b[38;2;239;68;68m",
        reset = Zeta::RESET,
    );
    eprintln!();
}

// ─── Info ────────────────────────────────────────────────────────────────────

pub fn info(msg: &str) {
    println!("  {muted}ℹ {msg}{reset}", muted = Zeta::MUTED, reset = Zeta::RESET);
}

// ─── Box divider ────────────────────────────────────────────────────────────

pub fn divider() {
    println!("  {muted}{} {reset}", "─".repeat(42), muted = Zeta::MUTED, reset = Zeta::RESET);
}



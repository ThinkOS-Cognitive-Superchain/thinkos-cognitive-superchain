use chrono::Local;
use colored::*;

fn banner() {
    println!("{}", "===============================================".bright_cyan());
    println!("{}", " ThinkOS Cognitive Superchain — Dev Boot ".bright_cyan().bold());
    println!("{}", " (CMPS/TCF/NMTP stubs)".bright_cyan());
    println!("{}", "===============================================".bright_cyan());
}

fn main() {
    banner();

    let now = Local::now();
    println!("Boot time: {}", now.format("%Y-%m-%d %H:%M:%S").to_string().white());

    // Demo: call into CMPS crate
    let s = thinkos_cmps::Scores { continuity: 0.9, cognition: 0.8, synergy: 0.7, adaptation: 0.6, integrity: 0.95 };
    let w = (0.25, 0.30, 0.20, 0.15, 0.10);
    let score = thinkos_cmps::composite(&s, w);
    println!("CMPS composite score: {:.4}", score);

    println!("{}", "Dev node exited gracefully (no network yet).".bright_green());
}

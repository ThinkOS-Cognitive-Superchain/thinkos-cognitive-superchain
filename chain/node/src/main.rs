use chrono::Local;
use colored::*;
use serde::{Deserialize, Serialize};

fn banner() {
    println!(
        "{}",
        "===============================================".bright_cyan()
    );
    println!(
        "{}",
        " ThinkOS Cognitive Superchain — Dev Boot "
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        " (CMPS/TCF/NMTP stubs + AIFA handshake + CTP)".bright_cyan()
    );
    println!(
        "{}",
        "===============================================".bright_cyan()
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct Telemetry {
    volatility: f64,
    congestion: f64,
    uptime_variance: f64,
    treasury_health: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weights {
    w0: f64,
    w1: f64,
    w2: f64,
    w3: f64,
    w4: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct VaultSplit {
    innovation: u32,
    governance: u32,
}

/// Simple helper: fetch weights from AIFA (blocking HTTP).
fn fetch_aifa_weights(aifa_url: &str, t: &Telemetry) -> anyhow::Result<Weights> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/weights", aifa_url.trim_end_matches('/'));
    let resp = client.post(&url).json(t).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("AIFA /weights returned {}", resp.status());
    }
    Ok(resp.json::<Weights>()?)
}

/// Fetch current vault split (Innovation/Governance) for a given market mode.
fn fetch_vault_split(aifa_url: &str, mode: &str) -> anyhow::Result<VaultSplit> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/vault_split", aifa_url.trim_end_matches('/'));
    let resp = client.get(&url).query(&[("mode", mode)]).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("AIFA /vault_split returned {}", resp.status());
    }
    Ok(resp.json::<VaultSplit>()?)
}

fn main() -> anyhow::Result<()> {
    banner();

    let now = Local::now();
    println!(
        "Boot time: {}",
        now.format("%Y-%m-%d %H:%M:%S").to_string().white()
    );

    // AIFA endpoint + market mode (override via env)
    let aifa_url =
        std::env::var("THINKOS_AIFA_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".into());
    let market_mode = std::env::var("THINKOS_MARKET_MODE").unwrap_or_else(|_| "neutral".into());
    println!("Using AIFA endpoint: {}", aifa_url.bright_yellow());
    println!(
        "Market mode (for vault split): {}",
        market_mode.bright_yellow()
    );

    // Demo telemetry — later this will be live network/treasury data.
    let t = Telemetry {
        volatility: 0.2,
        congestion: 0.1,
        uptime_variance: 0.02,
        treasury_health: 0.9,
    };

    // 1) Static CMPS score (previous behavior)
    let s = thinkos_cmps::Scores {
        continuity: 0.9,
        cognition: 0.8,
        synergy: 0.7,
        adaptation: 0.6,
        integrity: 0.95,
    };
    let w_static = (0.25, 0.30, 0.20, 0.15, 0.10);
    let score_static = thinkos_cmps::composite(&s, w_static);
    println!("CMPS composite score (static): {:.4}", score_static);

    // 2) Dynamic weights from AIFA
    match fetch_aifa_weights(&aifa_url, &t) {
        Ok(w) => {
            println!(
                "AIFA weights: w0={:.4}, w1={:.4}, w2={:.4}, w3={:.4}, w4={:.4}",
                w.w0, w.w1, w.w2, w.w3, w.w4
            );
            let score_dynamic = thinkos_cmps::composite(&s, (w.w0, w.w1, w.w2, w.w3, w.w4));
            println!(
                "{}",
                format!("CMPS composite score (AIFA): {:.4}", score_dynamic).bright_green()
            );
        }
        Err(e) => eprintln!(
            "{}",
            format!("AIFA /weights fetch failed: {e}").bright_red()
        ),
    }

    // 3) Vault split (CTP) from AIFA
    match fetch_vault_split(&aifa_url, &market_mode) {
        Ok(v) => {
            println!(
                "{}",
                format!(
                    "Protocol Reserve split — Innovation: {}% | Governance: {}%",
                    v.innovation, v.governance
                )
                .bright_cyan()
            );
        }
        Err(e) => eprintln!(
            "{}",
            format!("AIFA /vault_split fetch failed: {e}").bright_red()
        ),
    }

    println!(
        "{}",
        "Dev node exited gracefully (no network yet).".bright_green()
    );
    Ok(())
}

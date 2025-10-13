use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, thread, time::Duration};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

fn save_json<T: Serialize, P: AsRef<Path>>(value: &T, path: P) -> anyhow::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let s = serde_json::to_string_pretty(value)?;
    fs::write(path, s)?;
    Ok(())
}

fn post_weights(aifa: &str, tele: &Telemetry) -> anyhow::Result<Weights> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/weights", aifa.trim_end_matches('/'));
    let resp = client.post(url).json(tele).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("AIFA /weights {}", resp.status());
    }
    Ok(resp.json::<Weights>()?)
}

fn get_vault_split(aifa: &str, mode: &str) -> anyhow::Result<VaultSplit> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/vault_split", aifa.trim_end_matches('/'));
    let resp = client.get(url).query(&[("mode", mode)]).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("AIFA /vault_split {}", resp.status());
    }
    Ok(resp.json::<VaultSplit>()?)
}

fn composite(s: &thinkos_cmps::Scores, w: &Weights) -> f64 {
    thinkos_cmps::composite(s, (w.w0, w.w1, w.w2, w.w3, w.w4))
}

fn env<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}

fn banner(node_id: &str) {
    println!(
        "{}",
        "================================================".bright_cyan()
    );
    println!(
        "{}",
        format!(" ThinkOS Cognitive Superchain — Node {}", node_id)
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        " (CMPS + AIFA + Loop + Per-node persistence)".bright_cyan()
    );
    println!(
        "{}",
        "================================================".bright_cyan()
    );
}

fn main() -> anyhow::Result<()> {
    let node_id = std::env::var("THINKOS_NODE_ID").unwrap_or_else(|_| "A".into());
    banner(&node_id);

    let aifa_url =
        std::env::var("THINKOS_AIFA_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".into());
    let market_mode = std::env::var("THINKOS_MARKET_MODE").unwrap_or_else(|_| "neutral".into());
    let out_dir =
        std::env::var("THINKOS_STATE_DIR").unwrap_or_else(|_| format!("state/nodes/{node_id}"));
    let loop_secs: u64 = env("THINKOS_LOOP_SECS", 5);
    let iters: usize = env("THINKOS_ITERS", 60);

    // Telemetry per node (defaults are reasonable)
    let tele = Telemetry {
        volatility: env("THINKOS_TELE_VOL", 0.20_f64),
        congestion: env("THINKOS_TELE_CONG", 0.10_f64),
        uptime_variance: env("THINKOS_TELE_UPVAR", 0.02_f64),
        treasury_health: env("THINKOS_TELE_TREAS", 0.90_f64),
    };

    println!("AIFA: {}", aifa_url.bright_yellow());
    println!("Mode: {}", market_mode.bright_yellow());
    println!(
        "Node: {}  Out: {}  Loop: {}s  Iters: {}",
        node_id, out_dir, loop_secs, iters
    );
    println!(
        "Telemetry: vol={:.3} cong={:.3} upvar={:.3} treas={:.3}",
        tele.volatility, tele.congestion, tele.uptime_variance, tele.treasury_health
    );

    let s = thinkos_cmps::Scores {
        continuity: 0.90,
        cognition: 0.80,
        synergy: 0.70,
        adaptation: 0.60,
        integrity: 0.95,
    };

    for i in 1..=iters {
        let ts: DateTime<Utc> = Utc::now();
        print!(
            "{}",
            format!("[{ts}] [{}] tick {:03}  ", node_id, i).bright_black()
        );

        // fetch weights
        match post_weights(&aifa_url, &tele) {
            Ok(w) => {
                let score = composite(&s, &w);
                println!(
                    "{}",
                    format!(
                        "weights=({:.3},{:.3},{:.3},{:.3},{:.3}) score={:.4}",
                        w.w0, w.w1, w.w2, w.w3, w.w4, score
                    )
                    .white()
                );
                let snap = serde_json::json!({ "ts": ts.to_rfc3339(), "node": node_id, "telemetry": tele, "weights": w, "score": score });
                let _ = save_json(&snap, format!("{out_dir}/aifa_latest.json"));
            }
            Err(e) => println!("{}", format!("AIFA weights error: {e}").bright_red()),
        }

        // fetch vault split
        match get_vault_split(&aifa_url, &market_mode) {
            Ok(v) => {
                let snap = serde_json::json!({ "ts": ts.to_rfc3339(), "node": node_id, "mode": market_mode, "split": v });
                let _ = save_json(&snap, format!("{out_dir}/ctp_latest.json"));
            }
            Err(e) => println!("{}", format!("AIFA vault_split error: {e}").bright_red()),
        }

        thread::sleep(Duration::from_secs(loop_secs));
    }

    println!(
        "{}",
        format!("Node {} exiting after {} ticks.", node_id, iters).bright_green()
    );
    Ok(())
}

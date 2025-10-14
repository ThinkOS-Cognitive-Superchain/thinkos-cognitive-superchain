use anyhow::Context;
use chrono::{DateTime, Utc};
use colored::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, thread, time::Duration};

mod p2p;

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

fn main() -> anyhow::Result<()> {
    // --- node identity from env ---
    let node_id = std::env::var("NODE_ID")
        .unwrap_or_else(|_| "A".to_string())
        .chars()
        .next()
        .unwrap();

    // --- basic config ---
    let aifa = std::env::var("AIFA_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".into());
    let mode = std::env::var("MARKET_MODE").unwrap_or_else(|_| "neutral".into());
    let iters: u32 = std::env::var("ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
    let period_ms: u64 = std::env::var("PERIOD_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);

    // --- banner ---
    println!("{}", "===============================================".bright_cyan());
    println!("{}", " ThinkOS Cognitive Superchain — Dev Boot".bright_white());
    println!("{}", " (CMPS + AIFA + CTP + persistence)".bright_white());
    println!("{}", "===============================================".bright_cyan());
    println!("Boot: {}", Utc::now().to_rfc3339());
    println!("AIFA: {}", aifa);
    println!("Mode: {}", mode);

    // --- state dir per node ---
    let state_dir = std::path::PathBuf::from(format!("state/nodes/{}/", node_id));
    std::fs::create_dir_all(&state_dir).ok();

    // --- start lightweight UDP gossip in background ---
    p2p::spawn(node_id, state_dir.clone());

    let client = Client::builder().build()?;

    // one static composite (for display) using equal-ish weights
    let static_scores = thinkos_cmps::Scores {
        continuity: 0.9,
        cognition: 0.8,
        synergy: 0.7,
        adaptation: 0.6,
        integrity: 0.85,
    };
    let static_w = (0.25, 0.30, 0.20, 0.15, 0.10);
    let static_comp = thinkos_cmps::composite(&static_scores, static_w);
    println!("Static composite: {:.4}", static_comp);

    for t in 1..=iters {
        let ts: DateTime<Utc> = Utc::now();

        // 1) ask AIFA for dynamic weights
        let tel = Telemetry {
            volatility: 0.23,
            congestion: 0.18,
            uptime_variance: 0.03,
            treasury_health: 0.88,
        };

        let w_resp = client
            .post(format!("{}/weights", aifa))
            .json(&tel)
            .send();

        let weights: Option<Weights> = match w_resp {
            Ok(resp) if resp.status().is_success() => resp.json().ok(),
            _ => None,
        };

        // 2) ask AIFA for vault split
        let s_resp = client
            .get(format!("{}/vault_split", aifa))
            .query(&[("mode", mode.as_str())])
            .send();

        let split: Option<VaultSplit> = match s_resp {
            Ok(resp) if resp.status().is_success() => resp.json().ok(),
            _ => None,
        };

        // 3) compute dynamic composite if weights arrived
        let dyn_comp = if let Some(w) = &weights {
            let s = thinkos_cmps::Scores {
                continuity: 0.9,
                cognition: 0.8,
                synergy: 0.7,
                adaptation: 0.6,
                integrity: 0.85,
            };
            let w_tuple = (w.w0, w.w1, w.w2, w.w3, w.w4);
            Some(thinkos_cmps::composite(&s, w_tuple))
        } else {
            None
        };

        // 4) persist snapshots
        if let Some(w) = &weights {
            save_json(w, state_dir.join("aifa_latest.json")).ok();
        }
        if let Some(sp) = &split {
            save_json(sp, state_dir.join("ctp_latest.json")).ok();
        }

        // 5) log line
        match (weights, split, dyn_comp) {
            (Some(w), Some(sp), Some(dc)) => {
                println!(
                    "[{}] [{}] tick {:03}  weights=({:.3},{:.3},{:.3},{:.3},{:.3}) dyn={:.4} split=({}/{})",
                    ts,
                    node_id,
                    t,
                    w.w0, w.w1, w.w2, w.w3, w.w4,
                    dc,
                    sp.innovation, sp.governance,
                );
            }
            _ => {
                println!(
                    "[{}] [{}] tick {:03}  {}",
                    ts,
                    node_id,
                    t,
                    "AIFA request failed — continuing with last known values".bright_yellow()
                );
            }
        }

        thread::sleep(Duration::from_millis(period_ms));
    }

    println!(
        "{}",
        format!("Node {} exiting after {} ticks.", node_id, iters).bright_green()
    );
    Ok(())
}

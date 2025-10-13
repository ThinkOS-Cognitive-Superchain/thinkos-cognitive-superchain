from fastapi import FastAPI, HTTPException
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[2]  # repo root
STATE = ROOT / "state" / "nodes"

app = FastAPI(title="Cogniscan Lite")

def read_json(p: Path):
    with p.open("r", encoding="utf-8") as f:
        return json.load(f)

@app.get("/api/nodes")
def api_nodes():
    nodes = []
    for d in sorted(STATE.glob("*")):
        aifa = d / "aifa_latest.json"
        if aifa.exists():
            data = read_json(aifa)
            nodes.append({
                "node": data.get("node", d.name),
                "score": data.get("score"),
                "weights": data.get("weights"),
                "telemetry": data.get("telemetry"),
                "ts": data.get("ts"),
            })
    return {"nodes": nodes}

@app.get("/api/split")
def api_split():
    any_ctp = list(STATE.glob("*/ctp_latest.json"))
    if not any_ctp:
        raise HTTPException(404, "No CTP snapshots found")
    data = read_json(any_ctp[0])
    return {"mode": data.get("mode"), "split": data.get("split"), "ts": data.get("ts")}

INDEX_HTML = """
<!doctype html>
<html>
<head>
<meta charset="utf-8"/>
<title>Cogniscan Lite</title>
<style>
html,body{background:#000;color:#fff;font-family:ui-sans-serif,system-ui,Segoe UI,Roboto,Helvetica,Arial,sans-serif;margin:0;padding:0}
.wrap{max-width:1000px;margin:32px auto;padding:0 16px}
.grid{display:grid;grid-template-columns:1fr 1fr;gap:16px}
.card{background:#0a0a0a;border:1px solid #1e1e1e;border-radius:12px;padding:16px}
.h{font-weight:700;font-size:20px;margin:0 0 8px}
.k{color:#7dd3fc}
.small{color:#aaa;font-size:12px}
table{width:100%;border-collapse:collapse;margin-top:8px}
td,th{border-bottom:1px solid #1e1e1e;padding:8px;text-align:left}
.badge{display:inline-block;background:#0e7490;color:#d1faff;border-radius:999px;padding:4px 10px;font-size:12px}
</style>
</head>
<body>
<div class="wrap">
  <h1 style="margin:0 0 16px">ThinkOS Cogniscan <span class="badge">Lite</span></h1>
  <div id="split" class="card">
    <div class="h">Protocol Reserve Split</div>
    <div class="small">Mode: <span id="mode" class="k">—</span> • Updated: <span id="split_ts" class="k">—</span></div>
    <div style="margin-top:8px">Innovation: <span id="inv" class="k">—</span>% &nbsp;|&nbsp; Governance: <span id="gov" class="k">—</span>%</div>
  </div>

  <div class="grid" style="margin-top:16px">
    <div class="card">
      <div class="h">Node Scores</div>
      <table id="nodes"><thead><tr><th>Node</th><th>Score</th><th>Updated</th></tr></thead><tbody></tbody></table>
    </div>
    <div class="card">
      <div class="h">Current Weights (AIFA)</div>
      <table id="weights"><thead><tr><th>w0</th><th>w1</th><th>w2</th><th>w3</th><th>w4</th></tr></thead><tbody></tbody></table>
      <div class="small" style="margin-top:6px">Shows weights for the first node in the list.</div>
    </div>
  </div>
</div>

<script>
async function refresh() {
  const S = await fetch('/api/split').then(r=>r.json()).catch(()=>null);
  if (S) {
    document.getElementById('mode').textContent = S.mode ?? '—';
    document.getElementById('split_ts').textContent = S.ts ?? '—';
    document.getElementById('inv').textContent = S.split?.innovation ?? '—';
    document.getElementById('gov').textContent = S.split?.governance ?? '—';
  }

  const N = await fetch('/api/nodes').then(r=>r.json()).catch(()=>null);
  const tb = document.querySelector('#nodes tbody'); tb.innerHTML = '';
  const wb = document.querySelector('#weights tbody'); wb.innerHTML = '';
  if (N && N.nodes) {
    N.nodes.forEach(n=>{
      const tr = document.createElement('tr');
      const score = (typeof n.score === 'number') ? n.score.toFixed(4) : n.score ?? '—';
      tr.innerHTML = `<td>${n.node}</td><td>${score}</td><td>${n.ts??'—'}</td>`;
      tb.appendChild(tr);
    });
    if (N.nodes.length) {
      const w = N.nodes[0].weights || {};
      const fmt = (x)=> (typeof x === 'number') ? x.toFixed(3) : (x ?? '—');
      const tr = document.createElement('tr');
      tr.innerHTML = `<td>${fmt(w.w0)}</td><td>${fmt(w.w1)}</td><td>${fmt(w.w2)}</td><td>${fmt(w.w3)}</td><td>${fmt(w.w4)}</td>`;
      wb.appendChild(tr);
    }
  }
}
setInterval(refresh, 1500);
refresh();
</script>
</body></html>
"""
@app.get("/", response_class=HTMLResponse)
def index():
    return INDEX_HTML

app.mount("/static", StaticFiles(directory=Path(__file__).parent / "static"), name="static")

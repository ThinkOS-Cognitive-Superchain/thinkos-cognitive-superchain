from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(title="AIFA Engine (stub)")

class Telemetry(BaseModel):
    volatility: float = 0.3
    congestion: float = 0.2
    uptime_variance: float = 0.05
    treasury_health: float = 0.8

@app.get("/health")
def health():
    return {"ok": True}

@app.post("/weights")
def weights(t: Telemetry):
    # Very simple heuristic for demo
    w0 = 0.25 + 0.05*max(0.0, 0.5 - t.uptime_variance)
    w1 = 0.30 + 0.10*max(0.0, 1.0 - t.volatility)
    w2 = 0.20 + 0.05*max(0.0, 1.0 - t.congestion)
    w3 = 0.15 + 0.05*(1.0 - t.treasury_health)
    w4 = 0.10
    scale = w0+w1+w2+w3+w4
    return {"w0": w0/scale, "w1": w1/scale, "w2": w2/scale, "w3": w3/scale, "w4": w4/scale}

@app.get("/vault_split")
def vault_split(mode: str = "neutral"):
    table = {
        "bear":    {"innovation": 90, "governance": 10},
        "neutral": {"innovation": 80, "governance": 20},
        "bull":    {"innovation": 70, "governance": 30},
    }
    return table.get(mode, table["neutral"])

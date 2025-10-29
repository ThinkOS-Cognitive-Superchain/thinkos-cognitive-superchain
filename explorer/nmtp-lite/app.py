from fastapi import FastAPI, Request
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from fastapi.responses import JSONResponse
import random, asyncio

app = FastAPI(title="ThinkOS NMTP Hybrid Dashboard v4.0")

# Mount static files and templates
app.mount("/static", StaticFiles(directory="static"), name="static")
templates = Jinja2Templates(directory="templates")

@app.get("/")
def index(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})

# ---- API: Neural Mesh Simulation ----
@app.get("/api/mesh")
async def get_mesh():
    nodes = [
        {"id": f"N{i}", "energy": round(random.uniform(0.5, 1.0), 2), "connections": random.randint(2, 6)}
        for i in range(1, 12)
    ]
    flux = round(sum(n["energy"] for n in nodes) / len(nodes), 2)
    stability = round(random.uniform(0.6, 1.0), 2)
    power = round(flux * 6, 3)
    entropy = round(1 - stability, 2)

    await asyncio.sleep(0.3)
    return JSONResponse({
        "nodes": nodes,
        "flux": flux,
        "stability": stability,
        "power": power,
        "entropy": entropy,
        "aifa_message": random.choice([
            "Synchronizing neural resonance...",
            "Flux stable. Monitoring entropy.",
            "AIFA Link engaged.",
            "Quantum resonance approaching peak flux.",
            "Mesh harmonized with cognitive layer."
        ])
    })


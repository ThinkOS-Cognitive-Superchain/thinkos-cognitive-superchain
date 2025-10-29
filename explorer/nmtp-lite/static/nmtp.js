const meshCtx = document.getElementById("meshChart").getContext("2d");
const stabCtx = document.getElementById("stabilityChart").getContext("2d");

const meshChart = new Chart(meshCtx, {
  type: "line",
  data: { labels: [], datasets: [{ label: "Flux", borderColor: "#00ffff", data: [] }] },
  options: { scales: { y: { min: 0, max: 1 } } }
});

const stabChart = new Chart(stabCtx, {
  type: "doughnut",
  data: { labels: ["Stability", "Entropy"], datasets: [{ data: [0.9, 0.1], backgroundColor: ["#00ffff", "#1a1a1a"] }] },
  options: { cutout: "70%" }
});

async function refresh() {
  try {
    const res = await fetch("/api/mesh");
    const data = await res.json();

    const tbody = document.getElementById("nodeBody");
    tbody.innerHTML = data.nodes.map(n =>
      `<tr><td>${n.id}</td><td>${(n.energy*100).toFixed(1)}%</td><td>${n.connections}</td></tr>`
    ).join("");

    meshChart.data.labels.push(new Date().toLocaleTimeString());
    meshChart.data.datasets[0].data.push(data.flux);
    if (meshChart.data.labels.length > 20) {
      meshChart.data.labels.shift();
      meshChart.data.datasets[0].data.shift();
    }
    meshChart.update();

    stabChart.data.datasets[0].data = [data.stability, 1 - data.stability];
    stabChart.update();
  } catch (err) { console.error(err); }
}

setInterval(refresh, 1500);

// === INITIALIZE 3D SCENE ===
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(60, window.innerWidth / (window.innerHeight*0.7), 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById("neuralScene"), antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight * 0.7);
renderer.setPixelRatio(window.devicePixelRatio);
renderer.shadowMap.enabled = true;
renderer.outputEncoding = THREE.sRGBEncoding;
camera.position.set(0, 30, 150);

// === LIGHTING ===
const ambient = new THREE.AmbientLight(0x00eaff, 0.6);
scene.add(ambient);
const dirLight = new THREE.PointLight(0x00eaff, 1, 400);
dirLight.position.set(0, 50, 80);
scene.add(dirLight);

// === SKYLINE PARALLAX PLANE ===
const loader = new THREE.TextureLoader();
const skylineTex = loader.load('/static/textures/skyline.jpg');
const planeGeom = new THREE.PlaneGeometry(500, 200, 1, 1);
const planeMat = new THREE.MeshBasicMaterial({ map: skylineTex, side: THREE.DoubleSide, transparent: true, opacity: 0.45 });
const skyline = new THREE.Mesh(planeGeom, planeMat);
skyline.position.set(0, -60, -100);
skyline.rotation.x = -Math.PI/10;
scene.add(skyline);

// === SKYBOX ===
const cubeLoader = new THREE.CubeTextureLoader();
const textureCube = cubeLoader.load([
  '/static/textures/px.jpg',
  '/static/textures/nx.jpg',
  '/static/textures/py.jpg',
  '/static/textures/ny.jpg',
  '/static/textures/pz.jpg',
  '/static/textures/nz.jpg'
]);
scene.background = textureCube;

// === NEURAL NODES (3D GLOW) ===
const nodeGeo = new THREE.SphereGeometry(1.8, 16, 16);
const nodeMat = new THREE.MeshPhongMaterial({
  emissive: 0x00eaff,
  emissiveIntensity: 2,
  color: 0x001122
});
const nodes = [];
for (let i = 0; i < 40; i++) {
  const n = new THREE.Mesh(nodeGeo, nodeMat.clone());
  n.position.set((Math.random()-0.5)*120, (Math.random()-0.5)*80, (Math.random()-0.5)*100);
  scene.add(n);
  nodes.push(n);
}

// === CONNECTION LINES (HD) ===
const lineMat = new THREE.LineBasicMaterial({ color: 0x00eaff, transparent: true, opacity: 0.5 });
const lines = [];
for (let i=0; i<nodes.length; i++) {
  for (let j=i+1; j<nodes.length; j++) {
    const dist = nodes[i].position.distanceTo(nodes[j].position);
    if (dist < 45) {
      const pts = [nodes[i].position, nodes[j].position];
      const geo = new THREE.BufferGeometry().setFromPoints(pts);
      const line = new THREE.Line(geo, lineMat);
      scene.add(line);
      lines.push(line);
    }
  }
}

// === CAMERA CONTROLS (orbit drift) ===
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.enableZoom = false;
controls.autoRotate = true;
controls.autoRotateSpeed = 0.5;

// === AIFA SPEECH / UI DATA ===
const messages = [
  "Neural coherence optimal.",
  "Flux resonance synchronized.",
  "Entropy threshold balanced.",
  "AIFA network fully operational.",
  "Quantum harmonics aligned.",
  "Energy stability above threshold."
];

function simulateData() {
  return {
    stability: (Math.random()*0.15+0.85).toFixed(2),
    flux: (Math.random()*2).toFixed(2),
    power: (Math.random()*10).toFixed(2),
    message: messages[Math.floor(Math.random()*messages.length)]
  };
}

function updateUI() {
  const d = simulateData();
  document.getElementById("stabilityValue").textContent = d.stability;
  document.getElementById("powerValue").textContent = d.power;
  document.getElementById("fluxValue").textContent = d.flux;
  document.getElementById("aifaText").textContent = d.message;
  speak(d.message);
}
setInterval(updateUI, 5000);

function speak(text) {
  if (!window.speechSynthesis) return;
  window.speechSynthesis.cancel();
  const u = new SpeechSynthesisUtterance(text);
  u.lang = "en-US";
  u.pitch = 1.15;
  u.rate = 1.0;
  u.volume = 0.6;
  speechSynthesis.speak(u);
}

// === ANIMATION LOOP ===
function animate() {
  requestAnimationFrame(animate);

  // Node motion
  nodes.forEach(n => {
    n.position.x += (Math.random()-0.5)*0.05;
    n.position.y += (Math.random()-0.5)*0.05;
  });

  skyline.position.x = Math.sin(Date.now()*0.0002)*15;
  controls.update();
  renderer.render(scene, camera);
}
animate();

// === MODE SWITCHING ===
let currentMode = "SCM";
document.querySelectorAll(".modeBtn").forEach(btn => {
  btn.addEventListener("click", () => {
    document.querySelectorAll(".modeBtn").forEach(b => b.classList.remove("active"));
    btn.classList.add("active");
    currentMode = btn.dataset.mode;

    const color = currentMode === "QRE" ? 0xff00ff : currentMode === "SMI" ? 0x00ffb3 : 0x00eaff;
    nodes.forEach(n => n.material.emissive.setHex(color));
    lineMat.color.setHex(color);
    ambient.color.setHex(color);
    dirLight.color.setHex(color);

    document.body.style.background =
      currentMode === "QRE" ? "radial-gradient(circle at center, #0a0014, #000)"
      : currentMode === "SMI" ? "radial-gradient(circle at center, #00140a, #000)"
      : "radial-gradient(circle at center, #010a14, #000)";
  });
});

<script lang="ts">
import { initializedStore } from "$stores/initialized.svelte";
import { goto } from "$app/navigation";
import { platform } from "@tauri-apps/plugin-os";
import { onMount } from "svelte";
import { mode } from "mode-watcher";

let currentPlatform = $state("");
let canvasElement: HTMLCanvasElement;
let animationFrameId: number;
let ctx: CanvasRenderingContext2D;

async function initKittynode() {
  try {
    if (["ios", "android"].includes(currentPlatform)) {
      await initializedStore.fakeInitialize();
    } else {
      await initializedStore.initialize();
    }
  } catch (e) {
    alert(`Failed to initialize kittynode: ${e}`);
  }
  await goto("/");
}

interface Node {
  x: number;
  y: number;
  vx: number;
  vy: number;
}

onMount(() => {
  currentPlatform = platform();

  // Initialize the canvas animation
  const canvas = canvasElement;
  const context = canvas.getContext("2d");
  if (!context) {
    console.error("Failed to get 2D context");
    return;
  }
  ctx = context;

  const nodes: Node[] = [];
  const numNodes = 50; // todo: should be relative and reactive to scren size as well
  const maxVelocity = 0.75;
  const mouse = { x: null as number | null, y: null as number | null };

  function resizeCanvas() {
    const dpr = window.devicePixelRatio || 1;
    canvas.width = window.innerWidth * dpr;
    canvas.height = window.innerHeight * dpr;
    canvas.style.width = `${window.innerWidth}px`;
    canvas.style.height = `${window.innerHeight}px`;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0); // Reset transform and scale
  }

  resizeCanvas();
  window.addEventListener("resize", resizeCanvas);

  // Create nodes with random positions and velocities
  for (let i = 0; i < numNodes; i++) {
    nodes.push({
      x: Math.random() * window.innerWidth,
      y: Math.random() * window.innerHeight,
      vx: (Math.random() - 0.5) * maxVelocity,
      vy: (Math.random() - 0.5) * maxVelocity,
    });
  }

  // Mouse and touch event listeners
  function updateMousePosition(event: MouseEvent | TouchEvent) {
    if (event instanceof MouseEvent) {
      mouse.x = event.clientX;
      mouse.y = event.clientY;
    } else if (event instanceof TouchEvent) {
      const touch = event.touches[0];
      mouse.x = touch.clientX;
      mouse.y = touch.clientY;
    }
  }

  canvas.addEventListener("mousemove", updateMousePosition);
  canvas.addEventListener("touchmove", updateMousePosition);

  canvas.addEventListener("mouseleave", () => {
    mouse.x = null;
    mouse.y = null;
  });

  canvas.addEventListener("touchend", () => {
    mouse.x = null;
    mouse.y = null;
  });

  function animate() {
    // Set background color based on mode
    ctx.fillStyle = $mode === "dark" ? "#0a0a0a" : "#FFFFFF";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Update and draw nodes
    for (const node of nodes) {
      node.x += node.vx;
      node.y += node.vy;

      // Bounce off edges
      if (node.x <= 0 || node.x >= window.innerWidth) node.vx *= -1;
      if (node.y <= 0 || node.y >= window.innerHeight) node.vy *= -1;

      // Draw node
      ctx.beginPath();
      ctx.arc(node.x, node.y, 2, 0, Math.PI * 2);
      ctx.fillStyle = $mode === "dark" ? "#FFFFFF" : "#000000";
      ctx.fill();
    }

    // Draw connections
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const dx = nodes[i].x - nodes[j].x;
        const dy = nodes[i].y - nodes[j].y;
        const distance = Math.hypot(dx, dy);

        if (distance < 100) {
          ctx.beginPath();
          ctx.moveTo(nodes[i].x, nodes[i].y);
          ctx.lineTo(nodes[j].x, nodes[j].y);
          const color = $mode === "dark" ? "255, 255, 255" : "0, 0, 0";
          ctx.strokeStyle = `rgba(${color}, ${1 - distance / 100})`;
          ctx.lineWidth = 1;
          ctx.stroke();
        }
      }
    }

    // Interact with mouse/touch
    if (mouse.x !== null && mouse.y !== null) {
      for (const node of nodes) {
        const dx = node.x - mouse.x;
        const dy = node.y - mouse.y;
        const distance = Math.hypot(dx, dy);

        if (distance < 100) {
          const angle = Math.atan2(dy, dx);
          node.vx += Math.cos(angle) * 0.05;
          node.vy += Math.sin(angle) * 0.05;
        }
      }
    }

    animationFrameId = requestAnimationFrame(animate);
  }

  animate();

  return () => {
    // Cleanup on component unmount
    cancelAnimationFrame(animationFrameId);
    window.removeEventListener("resize", resizeCanvas);
    canvas.removeEventListener("mousemove", updateMousePosition);
    canvas.removeEventListener("touchmove", updateMousePosition);
  };
});
</script>

<style>
  canvas {
    position: fixed;
    top: 0;
    left: 0;
    z-index: -1;
  }
  .main-content {
    position: relative;
    z-index: 1;
  }
</style>

<canvas bind:this={canvasElement}></canvas>

<main class="flex flex-col justify-center items-center h-full text-center p-4 main-content">
  <button
    class="hover:scale-110 transition-transform duration-300"
    onclick={initKittynode}
  >
    <img
      class="logo w-24"
      src={`/images/kittynode-logo-circle.png`}
      alt="Kittynode Logo"
    />
  </button>
</main>

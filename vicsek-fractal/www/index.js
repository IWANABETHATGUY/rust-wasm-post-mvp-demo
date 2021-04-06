/**@type{HTMLCanvasElement} */
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const height = canvas.height;
const width = canvas.width;
ctx.fillStyle = "blue";
function drawFractal(x, y, w, h, depth, maxDepth) {
  if (depth === maxDepth) {
    ctx.rect(x, y, w, h);
    return;
  }
  if (w <= 1 || h <= 1) {
    ctx.rect(x, y, Math.max(w, 1), Math.max(h, 1));
    return;
  }
  const w_3 = w / 3;
  const h_3 = h / 3;
  drawFractal(x, y, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + 2 * w_3, y, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + w_3, y + h_3, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + 2 * w_3, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
}

let depth = 10;
console.time("js");
ctx.beginPath();
drawFractal(0, 0, width, height, 0, 10)
ctx.fill();
console.timeEnd("js");

WebAssembly.instantiateStreaming(fetch("/pkg/vicsek_fractal_bg.wasm")).then(({ instance }) => {
  console.time("wasm");
  instance.exports.draw_fractal(0, 0, width, height, 0, 100);
  const buffer_address = instance.exports.BUFFER.value;
  let data = new Uint8ClampedArray(instance.exports.memory.buffer, buffer_address, 900 * 900 * 4);
  ctx.putImageData(new ImageData(data, height, width), 0, 0);
  console.timeEnd("wasm");
  // console.log(buffer)
});

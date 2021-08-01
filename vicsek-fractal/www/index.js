import init, { dot_product, dot_product_simd,  } from "./pkg/vicsek_fractal";
/**@type{HTMLCanvasElement} */
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const height = canvas.height;
const width = canvas.width;
ctx.fillStyle = "red";
function drawFractal(x, y, w, h, depth, maxDepth) {
  if (depth === maxDepth) {
    ctx.rect(x, y, w, h);
    return;
  }
  if (w <= 1 || h <= 1) {
    ctx.rect(x, y, Math.max(w, 1), Math.max(h, 1));
    return;
  }
  const w_3 = ~~(w / 3);
  const h_3 = ~~(h / 3);
  drawFractal(x, y, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + 2 * w_3, y, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + w_3, y + h_3, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
  drawFractal(x + 2 * w_3, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
}

let depth = 8;

(async () => {
  let count = 13412003;
  let arr1 = [];
  let arr2 = [];
  for (let i = 0; i < count; i++) {
    arr1.push(Math.round(Math.random() * 100));
    arr2.push(Math.round(Math.random() * 100));
  }
  const wasm = await init();
  const a = new Int32Array(arr1);
  const b = new Int32Array(arr2);

  console.time("dot_prod");
  console.log(dot_product(a, b));
  console.timeEnd("dot_prod");

  console.time("dot_prod_simd");
  console.log(dot_product_simd(a, b));
  console.timeEnd("dot_prod_simd");

  console.time("js");
  let jsRes = 0;
  for (let i = 0; i < count; i++) {
    jsRes += arr1[i] * arr2[i];
  }
  console.log("js result", jsRes);
  console.timeEnd("js");

  console.time("wasm");
  wasm.draw_fractal(0, 0, width, height, 0, depth);
  console.timeEnd("wasm");
  // const buffer_address = instance.exports.BUFFER.value;
  // let data = new Uint8ClampedArray(instance.exports.memory.buffer, buffer_address, 900 * 900 * 4);
  // console.log(data);
  // ctx.putImageData(new ImageData(data, height, width), 0, 0);
})();

// console.time("js");
// ctx.beginPath();
// drawFractal(0, 0, width, height, 0, 10)
// ctx.fill();
// console.timeEnd("js");

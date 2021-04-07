import * as wasm from "../pkg/wasm_thread";

(async () => {
  await wasm.default();
  await wasm.initThreadPool(navigator.hardwareConcurrency / 2 - 1);
  let arr = new Array(5000000).fill(1);
  for (let i = 0; i < arr.length; i++) {
    arr[i] = Math.round(Math.random() * 10000);
  }
  let arrI32 = new Int32Array(arr);
  // arr.sort()
  // console.log(wasm.sort_array_concurrent(arrI32));

  // fibonacci bench
  console.time("js_fib");
  const js_fib = fibonacci(35);
  console.timeEnd("js_fib");
  console.time("wasm_parallel_fib");
  const wasm_parallel_fib = wasm.fibonacci_join(35);
  console.timeEnd("wasm_parallel_fib");
  console.time("wasm")
  const wasm_fib = wasm.fibonacci(35)
  console.timeEnd("wasm")
  console.assert(js_fib === wasm_parallel_fib && js_fib === wasm_fib)
})();

function fibonacci(n) {
  if (n < 2) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

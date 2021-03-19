import * as wasm from "../pkg/wasm_thread";

(async () => {
  await wasm.default();
  await wasm.initThreadPool(navigator.hardwareConcurrency - 2);
  let arr = new Array(5000000).fill(1);
  for (let i = 0; i < arr.length; i++) {
    arr[i] = Math.round(Math.random() * 10000);
  }
  let arrI32 = new Int32Array(arr);
  console.time("js");
  arr.sort()
  console.timeEnd("js");
  console.time("wasm");
  console.log(wasm.sort_array_concurrent(arrI32));
  console.timeEnd("wasm");
})();

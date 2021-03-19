import("./wasm_simd.js").then((module) => {
  console.time("wasm");
  let arr = new Float32Array(Array.from({ length: 1000000 }).fill(1));
  for (let i = 0; i < 10; i++) {
    module.example(arr);
  }
  console.timeEnd("wasm");
  //   console.log(arr);
  console.time("js");
  let arr2 = Array.from({ length: 100000 }).fill(1);
  for (let i = 0; i < 10; i++) {
    for (let i = 0; i < arr.length; i++) {
      arr2[i] *= 4;
    }
  }
  console.timeEnd("js");
});

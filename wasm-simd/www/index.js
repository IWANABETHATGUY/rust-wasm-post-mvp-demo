function copyMemory(data1, data2, instance) {
  // the `alloc` function returns an offset in
  // the module's memory to the start of the block
  var ptr1 = instance.alloc(data1.length);
  var ptr2 = instance.alloc(data2.length);
  // create a typed `ArrayBuffer` at `ptr` of proper size
  var mem1 = new Int32Array(instance.memory.buffer, ptr1, data1.length);
  var mem2 = new Int32Array(instance.memory.buffer, ptr2, data2.length);
  // copy the content of `data` into the memory buffer
  mem1.set(new Int32Array(data1));
  mem2.set(new Int32Array(data2));
  // return the pointer
  return [ptr1, ptr2];
}
function arraySum(arr1, arr2, instance) {
  // copy the contents of `array` into the
  // module's memory and get the offset
  let [ptr1, ptr2] = copyMemory(arr1, arr2, instance);

  // invoke the module's `array_sum` exported function
  // and log the result
  console.time("wasm");
  var res = instance.array_sum_simd(ptr1, ptr2, arr1.length, arr2.length);
  console.timeEnd("wasm");

  console.time("wasm_auto");
  var res2 = instance.array_sum(ptr1, ptr2, arr1.length, arr2.length);
  console.timeEnd("wasm_auto");
  return res;
}
(async () => {
  import("./wasm_simd.wasm").then(async (mod) => {
    // instantiate the module
    let count = 5_000_000;
    let arr1 = [];
    let arr2 = [];
    for (let i = 0; i < count; i++) {
      arr1.push(Math.round(Math.random() * 10));
      arr2.push(Math.round(Math.random() * 10));
    }
    var res = arraySum(arr1, arr2, mod);
    console.log("wasm-simd-dot-product: ", res);
    //   console.log(arr);
    console.time("js");
    let jsRes = 0;
    for (let i = 0; i < count; i++) {
      jsRes += arr1[i] * arr2[i];
    }
    console.timeEnd("js");
    console.log("js-dot-product:", jsRes);
  });
})();

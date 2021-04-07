mod utils;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sort_array_concurrent(arr: &mut [i32]) {
    arr.par_sort_unstable()
}
#[wasm_bindgen]
pub fn fibonacci_join(n: u32) -> u32 {
    if n < 20 {
        return fibonacci(n);
    }
    let (a, b) = rayon::join(|| fibonacci_join(n - 1), || fibonacci_join(n - 2));
    return a + b;
}
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 1,
        0 => 0,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
pub use wasm_bindgen_rayon::init_thread_pool;

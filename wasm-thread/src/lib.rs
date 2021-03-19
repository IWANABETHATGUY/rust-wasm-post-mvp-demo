mod utils;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sort_array_concurrent(arr: &mut [i32])  {
    arr.par_sort_unstable()
}
pub use wasm_bindgen_rayon::init_thread_pool;

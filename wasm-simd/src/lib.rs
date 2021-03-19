#![feature(wasm_target_feature)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
// #[cfg(target_arch = "wasm32")]
// #[target_feature(enable = "simd128")]
// #[no_mangle]
// pub fn example(x: &mut [f32; 1024], y: &mut [f32; 1024]) -> f32 {
//     x.iter().zip(y.iter()).map(|(x, y)| x * y).sum()
// }
#[wasm_bindgen]
pub fn example(x: &mut [f32]) {
    assert!(x.len() % 4 == 0);
    for x in x.iter_mut() {
        *x *= 4.0;
    }
}

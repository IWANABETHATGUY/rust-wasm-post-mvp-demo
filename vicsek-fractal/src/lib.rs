mod utils;

use std::ops::IndexMut;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const HEIGHT: usize = 900;
const WIDTH: usize = 900;
#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
// function drawFractal(x, y, w, h, depth, maxDepth) {
//     if (depth === maxDepth) {
//         ctx.rect(x, y, w, h);
//         return;
//     }
//     if (w <= 1 || h <= 1) {
//         ctx.rect(x, y, Math.max(w, 1), Math.max(h, 1));
//         return;
//     }
//     const w_3 = w / 3;
//     const h_3 = h / 3;
//     drawFractal(x, y, w_3, h_3, depth + 1, maxDepth);
//     drawFractal(x + 2 * w_3, y, w_3, h_3, depth + 1, maxDepth);
//     drawFractal(x + w_3, y + h_3, w_3, h_3, depth + 1, maxDepth);
//     drawFractal(x, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
//     drawFractal(x + 2 * w_3, y + 2 * h_3, w_3, h_3, depth + 1, maxDepth);
// }
// RGBA
#[wasm_bindgen]
pub fn draw_fractal(x: usize, y: usize, w: usize, h: usize, depth: usize, max_depth: usize) {
    if depth == max_depth {
        fill(x, y, w, h);
        return;
    }
    if w <= 1 || h <= 1 {
        fill(x, y, w.max(1), h.max(1));
        return;
    }
    let w_3 = w / 3;
    let h_3 = h / 3;
    draw_fractal(x, y, w_3, h_3, depth + 1, max_depth);
    draw_fractal(x + 2 * w_3, y, w_3, h_3, depth + 1, max_depth);
    draw_fractal(x + w_3, y + h_3, w_3, h_3, depth + 1, max_depth);
    draw_fractal(x, y + 2 * h_3, w_3, h_3, depth + 1, max_depth);
    draw_fractal(x + 2 * w_3, y + 2 * h_3, w_3, h_3, depth + 1, max_depth);
}
#[wasm_bindgen]
pub fn fill(x: usize, y: usize, w: usize, h: usize) {
    for i in y..y + h {
        for j in x..x + w {
            let index = i * WIDTH + j;
            unsafe {
                *BUFFER.get_unchecked_mut(index) = 0xFF_00_00_FF;
            }
        }
    }
}


#[wasm_bindgen]
pub fn dot_product_simd(a: &[i32], b: &[i32]) -> i32 {
    use packed_simd::i32x4;

    assert_eq!(a.len(), b.len());
    // assert!(a.len() % 4 == 0);
    let res = a
        .chunks_exact(4)
        .map(i32x4::from_slice_unaligned)
        .zip(b.chunks_exact(4).map(i32x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<i32x4>();
    res.wrapping_sum()
}

#[wasm_bindgen]
pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    assert_eq!(a.len(), b.len());
    // assert!(a.len() % 4 == 0);
    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (aa, bb)| acc + aa * bb)
}

// #[no_mangle]
// pub unsafe fn array_sum(ptr1: *mut i32, ptr2: *mut i32, len1: usize, len2: usize) -> i32 {
//     let mut res = 0;
//     for i in 0..len1 {
//         let i = i as isize;
//         res += (*ptr1.offset(i) * *ptr2.offset(i));
//     }
//     res
// }

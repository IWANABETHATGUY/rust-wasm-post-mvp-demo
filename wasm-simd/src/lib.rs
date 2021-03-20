#![feature(wasm_simd)]
#![feature(wasm_target_feature)]

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[wasm_bindgen]
// pub fn fuck(x: &mut [i32]) {
//     for x in x.iter_mut() {
//         *x *= 4;
//     }
// }

// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe fn array_sum_simd(ptr1: *mut i32, ptr2: *mut i32, len1: usize, len2: usize) -> i32 {
    use std::arch::wasm32::*;
    // create a Vec<u8> from the pointer to the
    // linear memory and the length
    // let mut a1 = std::slice::from_raw_parts(ptr1, len1);
    // let mut a2 = std::slice::from_raw_parts(ptr2, len2);
    let mut res: v128 = i32x4_splat(0);
    for i in (0..len1).step_by(4) {
        let i = i as isize;
        res = i32x4_add(
            res,
            i32x4_mul(
                i32x4_const(
                    *ptr1.offset(i),
                    *ptr1.offset(i + 1),
                    *ptr1.offset(i + 2),
                    *ptr1.offset(i + 3),
                ),
                i32x4_const(
                    *ptr2.offset(i),
                    *ptr2.offset(i + 1),
                    *ptr2.offset(i + 2),
                    *ptr2.offset(i + 3),
                ),
            ),
        );
    }
    i32x4_extract_lane::<0>(res)
        + i32x4_extract_lane::<1>(res)
        + i32x4_extract_lane::<2>(res)
        + i32x4_extract_lane::<3>(res)
    // actually compute the sum and return it
    // data.iter().sum()
}

#[no_mangle]
pub unsafe fn array_sum(ptr1: *mut i32, ptr2: *mut i32, len1: usize, len2: usize) -> i32 {
    let mut res = 0;
    for i in 0..len1 {
        let i = i as isize;
        res += (*ptr1.offset(i) * *ptr2.offset(i));
    }
    res
}

#[no_mangle]
pub fn alloc(len: usize) -> *mut i32 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure that its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    std::mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    return ptr;
}

// #[wasm_bindgen]
// pub fn test(a: i32) -> i32 {
//     return 0
// }

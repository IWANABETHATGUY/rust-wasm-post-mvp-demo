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
    let ptr1v128 = ptr1 as *const v128;
    let ptr2v128 = ptr2 as *const v128;
    let mut res: v128 = i32x4_splat(0);
    let mut index = 0isize;
    // let mut count = 0isize;
    for _ in (0..len1).step_by(4) {
        res = i32x4_add(
            res,
            i32x4_mul(
                v128_load((ptr1v128).offset(index)),
                v128_load((ptr2v128).offset(index)),
            ),
        );
        index += 1;
    }
    i32x4_extract_lane::<0>(res)
        + i32x4_extract_lane::<1>(res)
        + i32x4_extract_lane::<2>(res)
        + i32x4_extract_lane::<3>(res)
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

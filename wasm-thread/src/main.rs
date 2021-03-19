use rayon::prelude::*;
fn main() {
    let mut rng = rand::thread_rng();
    let mut arr = vec![];
    for i in 0..10000000 {
        arr.push(rng.gen_range(0, 100));
    }
    let mut arr2 = arr.clone();
    let time = std::time::Instant::now();
    sum_array(&mut arr);
    println!("{:?}", time.elapsed());
    let time = std::time::Instant::now();
    sum_array_concurrent(&mut arr2);
    println!("{:?}", time.elapsed());
}

fn sum_array(arr: &mut [i32]) {
    arr.sort();
}

fn sum_array_concurrent(arr: &mut [i32]) {
    arr.par_sort();
}

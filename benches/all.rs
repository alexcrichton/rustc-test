#[macro_use]
extern crate rustc_test;

use rustc_test::Bencher;
use rustc_test::stats::Stats;

fn sum_three_items(b: &mut Bencher) {
    b.iter(|| {
        [1e20f64, 1.5f64, -1e20f64].sum();
    })
}

fn sum_many_f64(b: &mut Bencher) {
    let nums = [-1e30f64, 1e60, 1e30, 1.0, -1e60];
    let v = (0..500).map(|i| nums[i%5]).collect::<Vec<_>>();

    b.iter(|| {
        v.sum();
    })
}

bench_main! {
    sum_three_items,
    sum_many_f64,
}

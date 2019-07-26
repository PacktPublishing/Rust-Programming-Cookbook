#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;
use rand::prelude::*;
use std::cell::RefCell;
use use_rayon::{merge_sort_par, merge_sort_seq};

fn random_number_vec(size: usize) -> Vec<i64> {
    let mut v: Vec<i64> = (0..size as i64).collect();
    let mut rng = thread_rng();
    rng.shuffle(&mut v);
    v
}

thread_local!(static ITEMS: RefCell<Vec<i64>> = RefCell::new(random_number_vec(100_000)));

fn bench_seq(c: &mut Criterion) {
    c.bench_function("10k merge sort (sequential)", |b| {
        ITEMS.with(|item| b.iter(|| black_box(merge_sort_seq(&item.borrow()))));
    });
}

fn bench_par(c: &mut Criterion) {
    c.bench_function("10k merge sort (parallel)", |b| {
        ITEMS.with(|item| b.iter(|| black_box(merge_sort_par(&item.borrow()))));
    });
}
criterion_group!(benches, bench_seq, bench_par);

criterion_main!(benches);

#[macro_use]
extern crate criterion;
use concurrent_processing::{ssqe, ssqe_sequential, seq_count_alpha_nums, par_count_alpha_nums};
use criterion::{black_box, Criterion};
use std::cell::RefCell;
use rand::prelude::*;

const SEQ_LEN: usize = 1_000_000;
thread_local!(static ITEMS: RefCell<(Vec<f32>, Vec<f32>)> = {
    let y_values: (Vec<f32>, Vec<f32>) = (0..SEQ_LEN).map(|_| (random::<f32>(), random::<f32>()) )
    .unzip();
    RefCell::new(y_values)
});


const MAX_CHARS: usize = 100_000;
thread_local!(static CHARS: RefCell<String> = {
    let items: String = (0..MAX_CHARS).map(|_| random::<char>()).collect();
    RefCell::new(items)
});

fn bench_count_seq(c: &mut Criterion) {
    c.bench_function("Counting in sequence", |b| {
        CHARS.with(|item| b.iter(|| black_box(seq_count_alpha_nums(&item.borrow()))))
    });
}

fn bench_count_par(c: &mut Criterion) {
    c.bench_function("Counting in parallel", |b| {
        CHARS.with(|item| b.iter(|| black_box(par_count_alpha_nums(&item.borrow()))))
    });
}


fn bench_seq(c: &mut Criterion) {
    c.bench_function("Sequential vector operation", |b| {
        ITEMS.with(|y_values| {
            let y_borrowed = y_values.borrow();
            b.iter(|| black_box(ssqe_sequential(&y_borrowed.0, &y_borrowed.1)))
        })
    });
}

fn bench_par(c: &mut Criterion) {
    c.bench_function("Parallel vector operation", |b| {
        ITEMS.with(|y_values| {
            let y_borrowed = y_values.borrow();
            b.iter(|| black_box(ssqe(&y_borrowed.0, &y_borrowed.1)))
        })
    });
}

criterion_group!(benches, bench_seq, bench_par,bench_count_par, bench_count_seq);

criterion_main!(benches);

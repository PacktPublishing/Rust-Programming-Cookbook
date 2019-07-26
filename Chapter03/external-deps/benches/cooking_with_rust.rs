#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result: Vec<T> = collection.into();
    for _ in 0..result.len() {
        let mut swaps = 0;
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
                swaps += 1;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    result
}

fn bench_bubble_sort_1k_asc(c: &mut Criterion) {
    c.bench_function("Bubble sort 1k descending numbers", |b| {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| black_box(bubble_sort(&items)))
    });
}

criterion_group!(benches, bench_bubble_sort_1k_asc);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};

use leetcode_practice::lc_rating::lc_1130_squares_of_a_sorted_array::Solution;

fn bench_function() -> Vec<i32> {
    Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
}

fn criterion(c: &mut Criterion) {
    c.bench_function("sort_squares", |b| {
        b.iter(|| {
            bench_function()
        })
    });
}

criterion_group!(benches, criterion);
criterion_main!(benches);
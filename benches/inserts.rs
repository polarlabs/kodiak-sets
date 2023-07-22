use criterion::{black_box, criterion_group, criterion_main, Criterion};

use kodiak_sets::sequence::Sequence;

mod helper;
use helper::next_ascii_char;

fn seq_new_insert_at_last_index(n: usize) {
    let mut seq: Sequence<char> = Sequence::new();
    let mut i: usize = 0;

    while i < n {
        seq.insert(i, next_ascii_char('A', (i % 10) as u8).unwrap());
        i += 1;
    }
}

fn vew_new_insert_at_last_index(n: usize) {
    let mut vec: Vec<char> = Vec::new();
    let mut i: usize = 0;

    while i < n {
        vec.insert(i, next_ascii_char('A', (i % 10) as u8).unwrap());
        i += 1;
    }
}

fn seq_with_capacity_insert_at_last_index(n: usize) {
    let mut seq: Sequence<char> = Sequence::with_capacity(n);
    let mut i: usize = 0;

    while i < n {
        seq.insert(i, next_ascii_char('A', (i % 10) as u8).unwrap());
        i += 1;
    }
}

#[allow(unused)]
fn vec_with_capacity_insert_at_last_index(n: usize) {
    let mut vec: Vec<char> = Vec::with_capacity(n);
    let mut i: usize = 0;

    while i < n {
        vec.insert(i, next_ascii_char('A', (i % 10) as u8).unwrap());
        i += 1;
    }
}

fn bench_seq_new_vs_with_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_new_vs_with_capacity");
    let n = 1_000_000;

    group.bench_function(format!("seq new - insert {} chars", n).as_str(), |b| b.iter(|| seq_new_insert_at_last_index(black_box(n))));
    group.bench_function(format!("seq with capacity - insert {} chars", n).as_str(), |b| b.iter(|| seq_with_capacity_insert_at_last_index(black_box(n))));
}

fn bench_seq_vs_vec_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_seq_vs_vec_new");
    let n = 1_000_000;

    group.bench_function(format!("vec new - insert {} chars", n).as_str(), |b| b.iter(|| vew_new_insert_at_last_index(black_box(n))));
    group.bench_function(format!("seq new - insert {} chars", n).as_str(), |b| b.iter(|| seq_new_insert_at_last_index(black_box(n))));
}

criterion_group!(benches, bench_seq_new_vs_with_capacity, bench_seq_vs_vec_new);
criterion_main!(benches);

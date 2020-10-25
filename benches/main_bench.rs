use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{seq::SliceRandom, thread_rng};
use avl_map::map::AvlMap;
use std::mem::size_of;

fn gen_remove_tree(keys: &Vec<i32>, size: usize) {
    let mut root = AvlMap::new();
    for k in &keys[..size] {
        root.insert(*k, ());
    }
    for k in &keys[..size] {
        root.remove(k);
    }
    assert_eq!(0, root.len());
}

fn bench_tree(c: &mut Criterion) {
    let mut keys = (0..1024 * 1024).collect::<Vec<_>>();
    let mut rng = thread_rng();
    keys[..].shuffle(&mut rng);

    let mut group = c.benchmark_group("bench_tree");
    for size in (0..=20).map(|x| 1 << x) {
        group.throughput(Throughput::Bytes((size * size_of::<usize>()) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| gen_remove_tree(&keys, size));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tree);
criterion_main!(benches);

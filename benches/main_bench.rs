use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{seq::SliceRandom, thread_rng};
use avl_map::tree::Node;
use std::mem::size_of;

fn gen_remove_tree(keys: &Vec<i32>, size: usize) {
    let mut root: Option<Box<Node<i32, ()>>> = None;
    for k in &keys[..size] {
        root = match root {
            None => Some(Box::new(Node::new(k, ()))),
            Some(n) => Some(n.insert(k, ())),
        }
    }
    for k in &keys[..size] {
        root = root.unwrap().remove(k);
    }
    assert!(root.is_none());
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

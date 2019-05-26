#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use indexmap::IndexMap;
use mutable::MutMap;
use mutable::MutVec;
use rand::{thread_rng, Rng};
use std::hash::Hash;

trait VecTrait<T>: Default {
    fn push_value(&mut self, value: T);
}

impl<T> VecTrait<T> for Vec<T> {
    fn push_value(&mut self, value: T) {
        self.push(value);
    }
}

impl<T> VecTrait<T> for MutVec<T> {
    fn push_value(&mut self, value: T) {
        self.push(value);
    }
}

fn push_data<V>(n: u64)
where
    V: VecTrait<u64>,
{
    let mut vec = V::default();
    for i in 0..n {
        vec.push_value(i);
    }
}

trait MapTrait<K, V>: Default {
    fn insert_value(&mut self, key: K, value: V);
    fn remove_value(&mut self, key: &K);
}

impl<K, V> MapTrait<K, V> for IndexMap<K, V>
where
    K: Eq + Hash,
{
    fn insert_value(&mut self, key: K, value: V) {
        self.insert(key, value);
    }

    fn remove_value(&mut self, key: &K) {
        self.remove(key);
    }
}

impl<K, V> MapTrait<K, V> for MutMap<K, V>
where
    K: Eq + Hash,
{
    fn insert_value(&mut self, key: K, value: V) {
        self.insert(key, value);
    }

    fn remove_value(&mut self, key: &K) {
        self.remove(key);
    }
}

fn map_insert_rand_bench<M: MapTrait<u64, u64>>(n: u64, b: &mut criterion::Bencher) {
    let mut map = M::default();

    // setup
    let mut rng = thread_rng();

    for _ in 0..n {
        let i = rng.gen::<u64>() % n;
        map.insert_value(i, i);
    }

    // measure
    b.iter(|| {
        let k = rng.gen::<u64>() % n;
        map.insert_value(k, k);
        map.remove_value(&k);
    });

    black_box(map);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("push_1024_vec", |b| {
        b.iter(|| push_data::<Vec<u64>>(black_box(1024)))
    });
    c.bench_function("push_1024_mutvec", |b| {
        b.iter(|| push_data::<MutVec<u64>>(black_box(1024)))
    });
    c.bench_function("map_insert_1024_indexmap", |b| {
        map_insert_rand_bench::<IndexMap<u64, u64>>(black_box(1024), b)
    });
    c.bench_function("map_insert_1024_mutmap", |b| {
        map_insert_rand_bench::<MutMap<u64, u64>>(black_box(1024), b)
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use mutable::MutVec;

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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("push_1024_vec", |b| {
        b.iter(|| push_data::<Vec<u64>>(black_box(1024)))
    });
    c.bench_function("push_1024_mutvec", |b| {
        b.iter(|| push_data::<MutVec<u64>>(black_box(1024)))
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);

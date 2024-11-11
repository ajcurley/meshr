use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use meshr::geometry::{Intersects, Aabb, Triangle, Vector3};

/// AABB/Triangle intersection test benchmark
pub fn benchmark_intersects_aabb_triangle(c: &mut Criterion) {
    c.bench_function("AABB/Triangle Intersection", |b| {
        let aabb = Aabb::unit();
        let p = generate_vector3();
        let q = generate_vector3();
        let r = generate_vector3();
        let triangle = Triangle::new(p, q, r);

        b.iter(|| {
            aabb.intersects(&triangle);
        });
    });
}

/// Generate a random Vector3 in the range [-4, 4]
fn generate_vector3() -> Vector3 {
    let mut rng = rand::thread_rng();
    let x = (rng.gen::<f64>() - 0.5) * 4.;
    let y = (rng.gen::<f64>() - 0.5) * 4.;
    let z = (rng.gen::<f64>() - 0.5) * 4.;
    Vector3::new(x, y, z)
}

criterion_group!(
    benches,
    benchmark_intersects_aabb_triangle,
);

criterion_main!(benches);

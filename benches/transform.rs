use criterion::{black_box, criterion_group, criterion_main, Criterion};
use htr::convert_props_react;
use std::thread;

/// bench tranform prop conversion to react
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform/personal");
    let sample_count = 1000;
    let sample_title = format!("convert_props_react {} samples", sample_count);
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;

    group.sample_size(sample_count);
    group.bench_function(format!("simultaneous: {}", sample_title), |b| {
        b.iter(|| convert_props_react(&html.to_string()))
    });
    group.finish();
}

/// bench concurrent crawling between different libs parallel 10x
pub fn bench_speed_concurrent_x10(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform/personal");
    let sample_count = 1000;
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;
    let sample_title = format!("convert_props_react {} samples", sample_count);
    let concurrency_count: Vec<_> = (0..10).collect();

    group.sample_size(sample_count);
    group.bench_function(format!("concurrent10x: {}", sample_title), |b| {
        b.iter(|| {
            let threads: Vec<_> = concurrency_count
                .clone()
                .into_iter()
                .map(|_| {
                    thread::spawn(move || {
                        black_box(convert_props_react(&html.to_string()));
                    })
                })
                .collect();

            for handle in threads {
                handle.join().unwrap()
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_speed, bench_speed_concurrent_x10);
criterion_main!(benches);

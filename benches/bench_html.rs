use std::collections::HashMap;

use criterion::{ criterion_group, criterion_main, Criterion };
use yew_benchmark_html_include::App;

fn setup() -> App {
    let mut data = HashMap::new();
    data.insert("foo".into(), "bar".into());
    App { data }
}

fn bench_html_func(c: &mut Criterion) {
    let app = setup();
    c.bench_function("view_html", |b|  b.iter(|| app.view_html()));
}

fn bench_html_include_func(c: &mut Criterion) {
    let app = setup();
    c.bench_function("view_html_include_internal", |b| b.iter(|| app.view_html_include()));
}

criterion_group!(benches, bench_html_func, bench_html_include_func);
criterion_main!(benches);
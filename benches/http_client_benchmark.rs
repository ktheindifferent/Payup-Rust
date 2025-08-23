use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;
use reqwest::Client;
use payup::http_client::{get_shared_client, get_shared_blocking_client};

/// Benchmark creating new clients vs using shared clients
fn bench_client_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("client_creation");
    
    // Benchmark creating new clients every time
    group.bench_function("new_client_per_request", |b| {
        b.iter(|| {
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .pool_idle_timeout(Duration::from_secs(90))
                .pool_max_idle_per_host(20)
                .build()
                .unwrap();
            black_box(client);
        });
    });
    
    // Benchmark using shared client
    group.bench_function("shared_client", |b| {
        b.iter(|| {
            let client = get_shared_client();
            black_box(client);
        });
    });
    
    group.finish();
}

/// Benchmark creating blocking clients
fn bench_blocking_client_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("blocking_client_creation");
    
    // Benchmark creating new blocking clients every time
    group.bench_function("new_blocking_client_per_request", |b| {
        b.iter(|| {
            let client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(30))
                .pool_idle_timeout(Duration::from_secs(90))
                .pool_max_idle_per_host(20)
                .build()
                .unwrap();
            black_box(client);
        });
    });
    
    // Benchmark using shared blocking client
    group.bench_function("shared_blocking_client", |b| {
        b.iter(|| {
            let client = get_shared_blocking_client();
            black_box(client);
        });
    });
    
    group.finish();
}

/// Benchmark concurrent client usage
fn bench_concurrent_usage(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;
    
    let mut group = c.benchmark_group("concurrent_usage");
    
    // Test with different concurrency levels
    for threads in &[1, 4, 8, 16] {
        // Benchmark creating new clients in concurrent scenario
        group.bench_with_input(
            BenchmarkId::new("new_clients", threads),
            threads,
            |b, &thread_count| {
                b.iter(|| {
                    let mut handles = vec![];
                    
                    for _ in 0..thread_count {
                        let handle = thread::spawn(|| {
                            let client = reqwest::blocking::Client::builder()
                                .timeout(Duration::from_secs(30))
                                .build()
                                .unwrap();
                            black_box(client);
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
        
        // Benchmark using shared client in concurrent scenario
        group.bench_with_input(
            BenchmarkId::new("shared_client", threads),
            threads,
            |b, &thread_count| {
                b.iter(|| {
                    let mut handles = vec![];
                    
                    for _ in 0..thread_count {
                        let handle = thread::spawn(|| {
                            let client = get_shared_blocking_client();
                            black_box(client);
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.sample_size(10); // Reduce sample size for memory-intensive benchmarks
    
    // Benchmark memory usage with multiple new clients
    group.bench_function("multiple_new_clients", |b| {
        b.iter(|| {
            let clients: Vec<_> = (0..100)
                .map(|_| {
                    Client::builder()
                        .timeout(Duration::from_secs(30))
                        .build()
                        .unwrap()
                })
                .collect();
            black_box(clients);
        });
    });
    
    // Benchmark memory usage with shared client clones
    group.bench_function("shared_client_clones", |b| {
        b.iter(|| {
            let clients: Vec<_> = (0..100)
                .map(|_| get_shared_client())
                .collect();
            black_box(clients);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_client_creation,
    bench_blocking_client_creation,
    bench_concurrent_usage,
    bench_memory_usage
);
criterion_main!(benches);
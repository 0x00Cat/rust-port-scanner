# Benchmarks for port scanner

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use port_scanner::prelude::*;
use std::net::IpAddr;
use std::time::Duration;

fn bench_scan_single_port(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_port_scan");
    
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    
    for port in [80, 443, 8080].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(port), port, |b, &port| {
            b.iter(|| {
                let config = ScanConfigBuilder::new()
                    .target(ip)
                    .custom_ports(vec![port])
                    .timeout(Duration::from_millis(100))
                    .parallel(false)
                    .build()
                    .unwrap();
                
                let scanner = PortScanner::new(config).unwrap();
                let _ = scanner.scan_all(|_| {});
            });
        });
    }
    
    group.finish();
}

fn bench_scan_port_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("port_range_scan");
    
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    
    for size in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let config = ScanConfigBuilder::new()
                    .target(ip)
                    .range(1, size)
                    .timeout(Duration::from_millis(50))
                    .parallel(true)
                    .thread_count(4)
                    .build()
                    .unwrap();
                
                let scanner = PortScanner::new(config).unwrap();
                let _ = scanner.scan_all(|_| {});
            });
        });
    }
    
    group.finish();
}

fn bench_parallel_vs_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_comparison");
    
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let port_count = 50;
    
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let config = ScanConfigBuilder::new()
                .target(ip)
                .range(1, port_count)
                .timeout(Duration::from_millis(50))
                .parallel(false)
                .build()
                .unwrap();
            
            let scanner = PortScanner::new(config).unwrap();
            let _ = scanner.scan_all(|_| {});
        });
    });
    
    group.bench_function("parallel", |b| {
        b.iter(|| {
            let config = ScanConfigBuilder::new()
                .target(ip)
                .range(1, port_count)
                .timeout(Duration::from_millis(50))
                .parallel(true)
                .thread_count(4)
                .build()
                .unwrap();
            
            let scanner = PortScanner::new(config).unwrap();
            let _ = scanner.scan_all(|_| {});
        });
    });
    
    group.finish();
}

fn bench_config_builder(c: &mut Criterion) {
    c.bench_function("config_builder", |b| {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        
        b.iter(|| {
            black_box(
                ScanConfigBuilder::new()
                    .target(ip)
                    .common_ports()
                    .timeout(Duration::from_millis(500))
                    .parallel(true)
                    .thread_count(8)
                    .build()
                    .unwrap()
            );
        });
    });
}

criterion_group!(
    benches,
    bench_scan_single_port,
    bench_scan_port_range,
    bench_parallel_vs_sequential,
    bench_config_builder
);

criterion_main!(benches);

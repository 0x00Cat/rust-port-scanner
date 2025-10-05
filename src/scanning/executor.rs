/// Parallel scanning implementation using rayon

use rayon::prelude::*;
use tracing::{info, debug};

use crate::domain::{Port, PortScanResult};
use crate::scanning::config::ScanConfig;
use crate::scanning::strategy::ScanStrategy;

/// Parallel scanning executor
pub struct ParallelExecutor {
    thread_count: usize,
}

impl ParallelExecutor {
    pub fn new(thread_count: usize) -> Self {
        Self { thread_count }
    }

    pub fn scan_ports<F>(
        &self,
        ports: Vec<Port>,
        strategy: &(dyn ScanStrategy + Sync),
        config: &ScanConfig,
        mut callback: F,
    ) -> Vec<PortScanResult>
    where
        F: FnMut(&PortScanResult) + Send,
    {
        info!("Starting parallel scan with {} threads", self.thread_count);
        
        // Configure rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_count)
            .build()
            .expect("Failed to create thread pool");

        // Use thread pool to scan ports
        let results: Vec<PortScanResult> = pool.install(|| {
            ports.par_iter()
                .map(|&port| {
                    debug!("Scanning port {}", port);
                    strategy.scan(port, config.target_ip, config)
                })
                .collect()
        });

        // Call callback for each result
        for result in &results {
            callback(result);
        }

        info!("Parallel scan completed. Scanned {} ports", results.len());
        results
    }
}

/// Sequential scanning executor
pub struct SequentialExecutor;

impl SequentialExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_ports<F>(
        &self,
        ports: Vec<Port>,
        strategy: &dyn ScanStrategy,
        config: &ScanConfig,
        mut callback: F,
    ) -> Vec<PortScanResult>
    where
        F: FnMut(&PortScanResult),
    {
        info!("Starting sequential scan");
        
        let mut results = Vec::with_capacity(ports.len());
        
        for port in ports {
            debug!("Scanning port {}", port);
            let result = strategy.scan(port, config.target_ip, config);
            callback(&result);
            results.push(result);
        }

        info!("Sequential scan completed. Scanned {} ports", results.len());
        results
    }
}

impl Default for SequentialExecutor {
    fn default() -> Self {
        Self::new()
    }
}

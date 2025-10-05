/// Async parallel scanning implementation using tokio

use tokio::task::JoinSet;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tracing::{info, debug};

use crate::domain::{Port, PortScanResult};
use crate::scanning::config::ScanConfig;
use crate::scanning::strategy::ScanStrategy;

/// Async parallel scanning executor with concurrency control
pub struct ParallelExecutor {
    max_concurrent: usize,
}

impl ParallelExecutor {
    pub fn new(max_concurrent: usize) -> Self {
        // Limit concurrency to reasonable bounds
        let max_concurrent = max_concurrent.min(2000).max(10);
        Self { max_concurrent }
    }

    pub async fn scan_ports<F>(
        &self,
        ports: Vec<Port>,
        strategy: Arc<dyn ScanStrategy + Send + Sync>,
        config: &ScanConfig,
        callback: F,
    ) -> Vec<PortScanResult>
    where
        F: Fn(&PortScanResult) + Send + Sync + 'static,
    {
        info!("Starting async parallel scan with max {} concurrent tasks", self.max_concurrent);
        
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let mut set = JoinSet::new();
        let callback = Arc::new(callback);
        let config = Arc::new(config.clone());

        // Spawn async tasks for each port
        for port in ports {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let strategy = Arc::clone(&strategy);
            let config = Arc::clone(&config);
            let callback = Arc::clone(&callback);

            set.spawn(async move {
                debug!("Scanning port {}", port);
                let result = strategy.scan_async(port, config.target_ip, &config).await;
                callback(&result);
                drop(permit); // Release semaphore
                result
            });
        }

        // Collect results
        let mut results = Vec::new();
        while let Some(res) = set.join_next().await {
            if let Ok(result) = res {
                results.push(result);
            }
        }

        info!("Async parallel scan completed. Scanned {} ports", results.len());
        results
    }
}

/// Sequential scanning executor (also async for consistency)
pub struct SequentialExecutor;

impl SequentialExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn scan_ports<F>(
        &self,
        ports: Vec<Port>,
        strategy: Arc<dyn ScanStrategy + Send + Sync>,
        config: &ScanConfig,
        callback: F,
    ) -> Vec<PortScanResult>
    where
        F: Fn(&PortScanResult),
    {
        info!("Starting sequential scan");
        
        let mut results = Vec::with_capacity(ports.len());
        
        for port in ports {
            debug!("Scanning port {}", port);
            let result = strategy.scan_async(port, config.target_ip, config).await;
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
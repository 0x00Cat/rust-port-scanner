/// Main port scanning use case (async)

use std::sync::Arc;
use tracing::{info, debug};

use crate::domain::{PortScanResult, ScanResults};
use crate::scanning::{ScanConfig, ScanStrategyFactory, ParallelExecutor, SequentialExecutor};
use crate::errors::ScanResult;

/// Port scanner orchestrator (async)
pub struct PortScanner {
    config: ScanConfig,
}

impl PortScanner {
    pub fn new(config: ScanConfig) -> ScanResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn config(&self) -> &ScanConfig {
        &self.config
    }

    /// Scan all configured ports (async)
    pub async fn scan_all<F>(&self, callback: F) -> ScanResults
    where
        F: Fn(&PortScanResult) + Send + Sync + 'static,
    {
        info!("Starting port scan on {}", self.config.target_ip);
        info!("Scan mode: {:?}", self.config.scan_mode);
        info!("Timeout: {:?}", self.config.timeout);
        info!("Parallel: {}", self.config.parallel);
        
        let ports = self.config.get_ports();
        info!("Total ports to scan: {}", ports.len());
        
        // Create the appropriate strategy
        let strategy = ScanStrategyFactory::create(&self.config);
        debug!("Using scan strategy: {}", strategy.name());
        
        // Execute async scan
        let results = if self.config.parallel {
            let executor = ParallelExecutor::new(self.config.thread_count * 4); // More concurrent tasks
            executor.scan_ports(ports, strategy, &self.config, callback).await
        } else {
            let executor = SequentialExecutor::new();
            executor.scan_ports(ports, strategy, &self.config, callback).await
        };
        
        info!("Scan completed. Total results: {}", results.len());
        ScanResults::from(results)
    }

    /// Scan a single port (async)
    pub async fn scan_port(&self, port: u16) -> PortScanResult {
        let strategy = ScanStrategyFactory::create(&self.config);
        strategy.scan_async(port, self.config.target_ip, &self.config).await
    }
}


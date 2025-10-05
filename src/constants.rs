/// Application constants

use std::time::Duration;

// Port ranges
pub const MIN_PORT: u16 = 1;
pub const MAX_PORT: u16 = 65535;
pub const HIGH_PORT_START: u16 = 1024;

// Default configuration values
pub const DEFAULT_TIMEOUT_MS: u64 = 500;
pub const DEFAULT_THREAD_COUNT: usize = 8;
pub const DEFAULT_VERBOSE: bool = false;
pub const DEFAULT_DETECT_VERSIONS: bool = false;
pub const DEFAULT_DETECT_OS: bool = false;
pub const DEFAULT_PARALLEL: bool = true;
pub const DEFAULT_RANDOMIZE_SOURCE: bool = false;

// Timeout durations
pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(DEFAULT_TIMEOUT_MS);
pub const BANNER_READ_TIMEOUT_MS: u64 = 2000;
pub const SMB_TIMEOUT_MS: u64 = 3000;

// Stealth settings
pub const DELAY_JITTER_PERCENT: u64 = 50;

// Buffer sizes
pub const BANNER_BUFFER_SIZE: usize = 1024;
pub const SMB_BUFFER_SIZE: usize = 4096;

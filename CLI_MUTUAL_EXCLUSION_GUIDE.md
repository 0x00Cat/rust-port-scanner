# CLI Mutual Exclusion Implementation Guide

## Overview
This guide explains how to add mutual exclusivity to command-line arguments using clap's argument groups.

## Changes Needed in `src/main_new.rs`

### 1. Add `ArgGroup` import
```rust
use clap::{Parser, ValueEnum, ArgGroup};
```

### 2. Add argument groups to the Cli struct

Replace the `#[command(...)]` attributes with:

```rust
#[derive(Parser, Debug)]
#[command(name = "port-scanner")]
#[command(author = "Your Name")]
#[command(version = "2.0.0")]
#[command(about = "A production-grade port scanner with service detection and OS fingerprinting", long_about = None)]
#[command(group(
    ArgGroup::new("port-spec")
        .required(false)
        .args(&["ports", "common"])
))]
struct Cli {
    /// Target IP address to scan
    #[arg(short, long, value_name = "IP")]
    target: Option<String>,

    /// Ports to scan (e.g., "80,443,8080" or "1-1000")
    #[arg(short, long, value_name = "PORTS", group = "port-spec")]
    ports: Option<String>,

    /// Use common ports preset
    #[arg(short, long, group = "port-spec")]
    common: bool,

    // ... rest of the fields remain the same ...
}
```

## What This Does

1. **Creates an argument group** named "port-spec" containing `--ports` and `--common`
2. **Makes them mutually exclusive**: Only one can be specified at a time
3. **Not required**: If neither is specified, the scanner uses a default (common ports)

## Benefits

### Before (Without Groups):
```bash
# This would be confusing - which option should be used?
port-scanner --target 127.0.0.1 --ports "80,443" --common
```

### After (With Groups):
```bash
# This will produce an error with a helpful message
port-scanner --target 127.0.0.1 --ports "80,443" --common
# Error: the argument '--ports <PORTS>' cannot be used with '--common'

# Valid usage:
port-scanner --target 127.0.0.1 --ports "80,443"
port-scanner --target 127.0.0.1 --common
```

## Additional Mutual Exclusion Suggestions

### 1. Output Format Groups
If you want only one output format at a time:

```rust
#[command(group(
    ArgGroup::new("output-format")
        .required(false)
        .args(&["format"])
        .multiple(false)
))]
```

### 2. Stealth Options
Group stealth options that might conflict:

```rust
#[command(group(
    ArgGroup::new("stealth")
        .required(false)
        .args(&["randomize_port", "delay"])
        .multiple(true)  // Can use both together
))]
```

## Complete Example

```rust
use clap::{Parser, ValueEnum, ArgGroup};

#[derive(Parser, Debug)]
#[command(name = "port-scanner")]
#[command(version = "2.0.0")]
#[command(about = "A production-grade port scanner")]
#[command(group(
    ArgGroup::new("port-spec")
        .required(false)
        .args(&["ports", "common"])
))]
struct Cli {
    #[arg(short, long)]
    target: Option<String>,

    #[arg(short, long, group = "port-spec")]
    ports: Option<String>,

    #[arg(short, long, group = "port-spec")]
    common: bool,

    #[arg(short = 'v', long)]
    detect_versions: bool,

    #[arg(short = 'o', long)]
    detect_os: bool,

    #[arg(long, default_value = "true")]
    parallel: bool,

    #[arg(short = 'T', long)]
    threads: Option<usize>,

    #[arg(long, default_value = "500")]
    timeout: u64,

    #[arg(long)]
    randomize_port: bool,

    #[arg(long)]
    delay: Option<u64>,

    #[arg(short = 'f', long, value_enum)]
    format: Option<OutputFormatArg>,

    #[arg(short = 'F', long)]
    output_file: Option<String>,

    #[arg(long)]
    verbose: bool,

    #[arg(long)]
    non_interactive: bool,

    #[arg(long)]
    open_only: bool,
}
```

## Testing

After implementing, test with:

```powershell
# Should work fine
cargo run -- --target 127.0.0.1 --common
cargo run -- --target 127.0.0.1 --ports "80,443"

# Should produce an error
cargo run -- --target 127.0.0.1 --common --ports "80,443"
```

## Error Messages

With mutual exclusivity, users get helpful error messages:

```
error: the argument '--ports <PORTS>' cannot be used with '--common'

Usage: port-scanner --target <IP> [--ports <PORTS> | --common]

For more information, try '--help'.
```

## Implementation Steps

1. Add `ArgGroup` to clap imports
2. Add `#[command(group(...))]` attribute to Cli struct  
3. Add `group = "port-spec"` to the `ports` and `common` field attributes
4. Build and test
5. Verify error messages are clear and helpful

## Related Documentation

- clap ArgGroup: https://docs.rs/clap/latest/clap/struct.ArgGroup.html
- clap Derive: https://docs.rs/clap/latest/clap/_derive/index.html

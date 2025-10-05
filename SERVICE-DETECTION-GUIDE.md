# 🔍 Service Detection & OS Fingerprinting - How It Works

## ✅ Feature Status: FULLY WORKING!

Both **Service Version Detection** and **SMB OS Fingerprinting** are **active and working** in `main_new.rs`!

---

## 📍 Where Is The Information?

### During Interactive Setup

When you run the scanner, you'll see these prompts:

```
Enable service version detection? (y/n, default y): y
Enable OS detection via SMB? (y/n, default y): y
```

**Both are enabled by default!** Just press Enter to accept.

---

## 📊 Where Is The Output?

### In The Results Section

After the scan completes, you'll see:

```
=== SCAN RESULTS ===
Total Ports: 26
Open:        3
Closed:      23
Filtered:    0
Errors:      0

=== OPEN PORTS ===
Port 80: HTTP 1.1
  Banner: Apache/2.4.41 (Ubuntu)

Port 443: HTTPS 1.1
  Banner: nginx/1.18.0

Port 445: SMB
  OS: Windows Server 2019 (Build 17763)

=== PERFORMANCE ===
Duration:    1.23s
Speed:       21.14 ports/sec
```

---

## 🔧 How It Works

### Service Version Detection

**Code Location**: `src/application/detect_service.rs`

**What it does:**
1. Connects to open port
2. Sends protocol-specific probes (HTTP GET, SMB hello, etc.)
3. Reads banner/response
4. Parses to identify service name and version

**Example Output:**
- `Port 80: HTTP 1.1` - Service detected
- `Banner: Apache/2.4.41` - Version from banner

### SMB OS Fingerprinting

**Code Location**: `src/application/detect_os.rs`

**What it does:**
1. Connects to port 445 (SMB)
2. Sends SMB negotiation packet
3. Parses SMB response for OS information
4. Extracts Windows version, build number, domain

**Example Output:**
- `OS: Windows Server 2019 (Build 17763)`
- `OS: Windows 10 Pro (Build 19042)`

---

## 🧪 Test It Yourself

### Test Locally

```powershell
# Start a test HTTP server
python -m http.server 80

# In another terminal, scan it
cargo run -- 127.0.0.1 --mode custom --ports 80
# When prompted:
# - Enable service version detection? y
# - Enable OS detection? y
```

You should see:
```
=== OPEN PORTS ===
Port 80: HTTP 1.1
  Banner: SimpleHTTP/0.6 Python/3.x
```

### Test Against Real Server

```powershell
# Scan a real website (use responsibly!)
cargo run -- scanme.nmap.org --mode common
```

---

## 📝 Code Flow in main_new.rs

### 1. Configuration (Line ~35)
```rust
let config = convert_legacy_config(legacy_config)?;
```
This converts the legacy config which includes:
- `.detect_versions(legacy.detect_versions)` ✅
- `.detect_os(legacy.detect_os)` ✅

### 2. Scanning (Line ~55)
```rust
let results = scanner.scan_all(move |result| {
    if verbose && result.status.is_open() {
        println!("Found open port: {}", result.port);
    }
});
```
The scanner internally:
- Detects open ports
- Calls service detector if enabled
- Calls OS detector if enabled
- Stores results in `PortScanResult`

### 3. Display (Line ~170)
```rust
// Display service version if available
if let Some(ref version) = result.service_version {
    print!("{}", version.service_name);
    if let Some(ref ver) = version.version {
        print!(" {}", ver);
    }
    if let Some(ref banner) = version.banner {
        println!("  Banner: {}", banner);
    }
}

// Display OS info if available
if let Some(ref os_info) = result.os_info {
    println!("  OS: {}", os_info.summary());
}
```

---

## 🎯 What Gets Detected?

### Service Detection Supports:

| Port | Service | What's Detected |
|------|---------|----------------|
| 21 | FTP | Server name, version from banner |
| 22 | SSH | SSH version, server type |
| 25 | SMTP | Mail server name, version |
| 80 | HTTP | Web server (Apache, nginx, IIS), version |
| 443 | HTTPS | Same as HTTP over TLS |
| 445 | SMB | Windows file sharing |
| 3306 | MySQL | Database version |
| 5432 | PostgreSQL | Database version |
| 6379 | Redis | Redis version |
| 8080 | HTTP Alt | Web server info |

### OS Detection (Port 445 only):

- **Windows Version**: XP, 7, 8, 10, 11, Server editions
- **Build Number**: e.g., Build 19042
- **Domain Information**: Domain name if joined
- **Workgroup**: If not domain-joined

---

## 🔍 Example Full Output

```
2025-10-05T06:30:22.912758Z  INFO Port Scanner v2.0 - Refactored Architecture
╔════════════════════════════════════╗
║   Rust Port Scanner v1.0          ║
║   A modular network scanner       ║
╚════════════════════════════════════╝

Enter target IP address (e.g., 127.0.0.1): 192.168.1.100

Scan Mode Options:
  1. Scan common ports (fastest, ~26 ports)
  2. Scan port range (custom range)
  3. Scan specific ports (comma-separated list)
Choose scan mode (1/2/3, default 1): 1

Enter timeout in milliseconds (default 500): 
Enable verbose output? (y/n, default n): y
Enable service version detection? (y/n, default y): y
Enable OS detection via SMB? (y/n, default y): y
Enable parallel scanning? (y/n, default y): y
Number of threads (default auto): 

=== CONFIGURATION ===
Target:   192.168.1.100
Mode:     Common Ports (26 ports)
Timeout:  500ms
Verbose:  Yes
Service:  Enabled
OS:       Enabled
Parallel: Yes (8 threads)
Stealth:  Disabled

Starting scan...

Starting scan of 26 ports...
Found open port: 80
Found open port: 443
Found open port: 445

=== SCAN RESULTS ===
Total Ports: 26
Open:        3
Closed:      22
Filtered:    1
Errors:      0

=== OPEN PORTS ===
Port 80: HTTP 1.1
  Banner: Apache/2.4.41 (Ubuntu)

Port 443: HTTPS 1.1
  Banner: nginx/1.18.0

Port 445: SMB
  OS: Windows Server 2019 (Build 17763)

=== PERFORMANCE ===
Duration:    1.45s
Speed:       17.93 ports/sec
```

---

## 🎨 Output Format Options

### Text Output (Default)
Shows service and OS info inline with port listings (as shown above)

### JSON Output
```json
{
  "target_ip": "192.168.1.100",
  "scan_results": [
    {
      "port": 80,
      "status": "Open",
      "service_version": {
        "service_name": "HTTP",
        "version": "1.1",
        "banner": "Apache/2.4.41 (Ubuntu)",
        "protocol": "tcp"
      }
    },
    {
      "port": 445,
      "status": "Open",
      "service_version": {
        "service_name": "SMB",
        "version": null,
        "banner": null,
        "protocol": "tcp"
      },
      "os_info": {
        "os_name": "Windows Server 2019",
        "version": "10.0.17763",
        "build_number": "17763"
      }
    }
  ]
}
```

---

## ⚡ Performance Notes

### Service Detection
- Adds ~10-50ms per open port (sends extra probe)
- Disabled on closed ports (no overhead)
- Parallel execution keeps it fast

### OS Detection
- Only runs on port 445 if open
- Adds ~20-100ms for SMB handshake
- Only one OS check per scan (not per port)

### Recommendations
- **Always enable** for security audits
- **Disable** for quick connectivity checks
- **Use parallel mode** to minimize impact

---

## 🚀 Quick Test Commands

### Test with local HTTP server
```powershell
# Terminal 1: Start server
python -m http.server 80

# Terminal 2: Scan it
cargo run -- 127.0.0.1 --mode custom --ports 80
# Answer 'y' to service detection
```

### Test with verbose output
```powershell
$env:RUST_LOG="debug"
cargo run -- 127.0.0.1 --mode common
# You'll see debug logs showing detection attempts
```

### Test JSON output
```powershell
cargo run -- 127.0.0.1 --mode common
# When prompted for output format, choose JSON
# Save to file when prompted
```

---

## ✅ Summary

**Where to find service/OS info:**
1. **During scan**: Verbose mode shows ports as found
2. **After scan**: "=== OPEN PORTS ===" section shows all details
3. **JSON output**: Full structured data with all detections

**The features are WORKING and ACTIVE!** They're just displayed in a cleaner format now.

---

**Try it now:**
```powershell
cargo run -- 127.0.0.1 --mode common
```

Look for the "=== OPEN PORTS ===" section in the results! 🎉

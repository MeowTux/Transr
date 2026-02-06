# TransR Functions & Plugins Documentation

Dokumentasi lengkap untuk semua functions dan plugins yang tersedia di TransR.

## 📋 Table of Contents

1. [Pipeline System](#pipeline-system)
2. [System Functions](#system-functions)
3. [Time Functions](#time-functions)
4. [Math Functions](#math-functions)
5. [Network Functions](#network-functions)
6. [Cryptography](#cryptography)
7. [Security Plugins](#security-plugins)

---

## 🔄 Pipeline System

### PipelineR Class

Pipeline system yang memungkinkan Anda menjalankan multiple tasks secara berurutan.

#### Methods

##### `__init__()`
Membuat instance PipelineR baru.
```python
from transR import PipelineR
p = PipelineR()
```

##### `set_verbose(level: int)`
Set verbose level (0-3).
- `0`: Silent - tidak ada output
- `1`: Info - informasi dasar
- `2`: Debug - informasi detail
- `3`: Trace - sangat detail

```python
p.set_verbose(2)
```

##### `set_debug(enabled: bool)`
Enable/disable debug mode untuk benchmarking.
```python
p.set_debug(True)
```

##### `print(text: str)`
Print text ke output.
```python
p.print("Hello, World!")
```

##### `system()`
Check system information.
```python
p.system()
```

##### `time()`
Check current time and date.
```python
p.time()
```

##### `math(operation: str, args: list[float])`
Perform math operation.
- Operations: `"sum"`, `"mean"`, `"max"`, `"min"`

```python
p.math("sum", [1, 2, 3, 4, 5])
p.math("mean", [10, 20, 30])
```

##### `http_get(url: str)`
Perform HTTP GET request.
```python
p.http_get("https://api.github.com")
```

##### `hash(algorithm: str, data: str)`
Hash data dengan algorithm tertentu.
- Algorithms: `"sha256"`, `"md5"`

```python
p.hash("sha256", "password123")
```

##### `loop_calc(iterations: int)`
Perform heavy calculation loop.
```python
p.loop_calc(1000000)
```

##### `blockchain(height: int, weight: int)`
Simulate blockchain-style random calculation.
```python
p.blockchain(100, 100)  # 10,000 iterations
```

##### `scan_ports(host: str, ports: list[int])`
Scan multiple ports.
```python
p.scan_ports("127.0.0.1", [80, 443, 8080])
```

##### `scan_vulns(url: str)`
Scan for vulnerabilities.
```python
p.scan_vulns("http://example.com")
```

##### `run()`
Execute all tasks in pipeline.
```python
p.run()
```

##### `clear()`
Clear all tasks and results.
```python
p.clear()
```

##### `results()`
Get execution results.
```python
results = p.results()
for r in results:
    print(f"{r['task']}: {r['output']} ({r['duration_ms']}ms)")
```

---

## 🖥️ System Functions

### Feature.system_info()

Mendapatkan informasi system lengkap.

**Returns:** `dict` dengan keys:
- `os_name`: Nama OS
- `os_version`: Versi OS
- `kernel`: Versi kernel
- `hostname`: Hostname
- `cpu_count`: Jumlah CPU cores
- `cpu_brand`: Nama/brand CPU
- `total_memory`: Total RAM (bytes)
- `used_memory`: RAM terpakai (bytes)
- `uptime`: System uptime (seconds)

**Example:**
```python
from transR import Feature
f = Feature()

info = f.system_info()
print(f"OS: {info['os_name']} {info['os_version']}")
print(f"CPU: {info['cpu_brand']} ({info['cpu_count']} cores)")
print(f"RAM: {info['used_memory'] / 1024**3:.2f}GB / {info['total_memory'] / 1024**3:.2f}GB")
```

### Feature.cpu_usage()

Mendapatkan CPU usage per core.

**Returns:** `list[float]` - CPU usage percentage untuk setiap core

**Example:**
```python
usage = f.cpu_usage()
print(f"CPU Usage: {usage}")
print(f"Average: {sum(usage)/len(usage):.1f}%")
```

### Feature.memory_percent()

Mendapatkan memory usage percentage.

**Returns:** `float` - Memory usage dalam persen

**Example:**
```python
mem = f.memory_percent()
print(f"Memory: {mem:.2f}%")
```

---

## ⏰ Time Functions

### Feature.now()

Mendapatkan waktu sekarang dengan detail lengkap.

**Returns:** `dict` dengan keys:
- `year`, `month`, `day`
- `hour`, `minute`, `second`
- `weekday`: Nama hari (Mon, Tue, ...)
- `formatted`: Formatted string
- `time_of_day`: Morning/Afternoon/Evening/Night

**Example:**
```python
now = f.now()
print(f"Date: {now['year']}-{now['month']:02d}-{now['day']:02d}")
print(f"Time: {now['hour']:02d}:{now['minute']:02d}:{now['second']:02d}")
print(f"Day: {now['weekday']}")
print(f"Period: {now['time_of_day']}")
```

### Feature.timestamp()

Mendapatkan Unix timestamp.

**Returns:** `int` - Unix timestamp (seconds)

**Example:**
```python
ts = f.timestamp()
print(f"Timestamp: {ts}")
```

### Feature.greeting()

Mendapatkan greeting berdasarkan waktu.

**Returns:** `str` - Greeting message

**Example:**
```python
print(f.greeting())
# Output: ☀️  Good Morning! / 🌤️  Good Afternoon! / etc.
```

---

## 🔢 Math Functions

### Basic Operations

#### Feature.sum(numbers: list[float])
```python
result = f.sum([1, 2, 3, 4, 5])  # 15.0
```

#### Feature.mean(numbers: list[float])
```python
result = f.mean([10, 20, 30])  # 20.0
```

#### Feature.sqrt(x: float)
```python
result = f.sqrt(16)  # 4.0
```

### Number Theory

#### Feature.factorial(n: int)
```python
result = f.factorial(5)  # "120"
```

#### Feature.is_prime(n: int)
```python
result = f.is_prime(17)  # True
result = f.is_prime(18)  # False
```

### Random Numbers

#### Feature.random()
```python
r = f.random()  # 0.0 to 1.0
```

#### Feature.random_range(min: int, max: int)
```python
r = f.random_range(1, 100)  # Random integer 1-100
```

---

## 🌐 Network Functions

### Feature.http_get(url: str)

Perform HTTP GET request.

**Parameters:**
- `url`: Target URL

**Returns:** `dict` dengan keys:
- `status`: HTTP status code
- `body`: Response body
- `url`: Final URL (after redirects)

**Example:**
```python
response = f.http_get("https://httpbin.org/get")
print(f"Status: {response['status']}")
print(f"Body: {response['body'][:100]}...")
```

---

## 🔐 Cryptography

### Feature.hash_sha256(data: str)

Hash data menggunakan SHA256.

**Parameters:**
- `data`: String to hash

**Returns:** `str` - Hex string hash

**Example:**
```python
hash_val = f.hash_sha256("password123")
print(f"SHA256: {hash_val}")
```

### Feature.hash_md5(data: str)

Hash data menggunakan MD5.

**Parameters:**
- `data`: String to hash

**Returns:** `str` - Hex string hash

**Example:**
```python
hash_val = f.hash_md5("hello")
print(f"MD5: {hash_val}")
```

---

## 🔍 Security Plugins

### RustNetX - Port Scanner

#### Feature.scan_port(host: str, port: int)

Scan single port.

**Parameters:**
- `host`: Target hostname/IP
- `port`: Port number

**Returns:** `dict` dengan keys:
- `host`: Target host
- `port`: Port number
- `is_open`: Boolean
- `service`: Service name (optional)

**Example:**
```python
result = f.scan_port("127.0.0.1", 80)
if result['is_open']:
    print(f"Port {result['port']} is OPEN ({result['service']})")
```

#### Feature.quick_scan(host: str)

Quick scan common ports.

**Parameters:**
- `host`: Target hostname/IP

**Returns:** `dict` dengan keys:
- `host`: Target host
- `is_alive`: Boolean
- `open_ports`: List of open ports
- `services`: List of services

**Example:**
```python
result = f.quick_scan("scanme.nmap.org")
print(f"Host alive: {result['is_alive']}")
print(f"Open ports: {result['open_ports']}")
for svc in result['services']:
    print(f"  - {svc}")
```

### HyrOS - Vulnerability Scanner

#### Feature.vuln_scan(url: str)

Scan for vulnerabilities.

**Parameters:**
- `url`: Target URL

**Returns:** `list[dict]` - List of vulnerabilities found

Each dict contains:
- `id`: Template ID
- `name`: Vulnerability name
- `severity`: Critical/High/Medium/Low/Info
- `url`: Tested URL
- `evidence`: Evidence (optional)

**Example:**
```python
vulns = f.vuln_scan("http://testphp.vulnweb.com")
for v in vulns:
    print(f"[{v['severity']}] {v['name']}")
    print(f"  URL: {v['url']}")
    if v['evidence']:
        print(f"  Evidence: {v['evidence'][:100]}...")
```

### Vulnerability Templates

HyrOS includes following vulnerability checks:

1. **Directory Traversal** (High)
   - Tests: `/../../../etc/passwd`
   - Looks for: `root:x:0:0`

2. **SQL Injection** (Critical)
   - Tests: `?id=1'`
   - Looks for: SQL error messages

3. **Reflected XSS** (High)
   - Tests: `?q=<script>alert(1)</script>`
   - Looks for: Script reflection

4. **Open Redirect** (Medium)
   - Tests: Redirect behavior
   - Checks: 302 status codes

5. **Exposed .git** (High)
   - Tests: `/.git/config`
   - Checks: 200 status

6. **Exposed .env** (Critical)
   - Tests: `/.env`
   - Looks for: `DB_PASSWORD`

7. **Debug Mode** (Medium)
   - Looks for: Debug flags

8. **Default Credentials** (Critical)
   - Tests: Default admin access

9. **API Exposure** (Info)
   - Tests: `/swagger-ui.html`
   - Looks for: API docs

10. **Missing Security Headers** (Low)
    - Checks: X-Content-Type-Options, etc.

---

## 🎯 Advanced Examples

### System Monitoring Script
```python
from transR import Feature
import time

f = Feature()

print("=== System Monitor ===")
while True:
    info = f.system_info()
    cpu = f.cpu_usage()
    mem = f.memory_percent()
    
    print(f"\rCPU: {sum(cpu)/len(cpu):5.1f}% | RAM: {mem:5.1f}% | Uptime: {info['uptime']}s", end="")
    time.sleep(1)
```

### Network Scanner Script
```python
from transR import Feature

f = Feature()

print("=== Network Scanner ===")
targets = ["192.168.1.1", "192.168.1.100", "192.168.1.254"]

for target in targets:
    print(f"\nScanning {target}...")
    result = f.quick_scan(target)
    
    if result['is_alive']:
        print(f"  ✓ Host is UP")
        print(f"  Open ports: {result['open_ports']}")
        for svc in result['services']:
            print(f"    - {svc}")
    else:
        print(f"  ✗ Host is DOWN")
```

### Security Audit Script
```python
from transR import Feature

f = Feature()

print("=== Security Audit ===")
target = "http://example.com"

print(f"Scanning {target}...")
vulns = f.vuln_scan(target)

critical = [v for v in vulns if v['severity'] == 'Critical']
high = [v for v in vulns if v['severity'] == 'High']
medium = [v for v in vulns if v['severity'] == 'Medium']
low = [v for v in vulns if v['severity'] == 'Low']

print(f"\nResults:")
print(f"  🔴 Critical: {len(critical)}")
print(f"  🟠 High: {len(high)}")
print(f"  🟡 Medium: {len(medium)}")
print(f"  🔵 Low: {len(low)}")

if critical:
    print("\n⚠️  CRITICAL VULNERABILITIES:")
    for v in critical:
        print(f"  - {v['name']}")
        print(f"    URL: {v['url']}")
```

### Data Processing Pipeline
```python
from transR import PipelineR
import json

p = PipelineR()
p.set_verbose(2)

# Fetch data
p.http_get("https://api.github.com/repos/rust-lang/rust")

# Process with math
data = [123, 456, 789]
p.math("mean", data)
p.math("sum", data)

# Generate hash
p.hash("sha256", str(data))

# Check system health
p.system()

# Run all
p.run()

# Get results
results = p.results()
print("\n=== Results ===")
for r in results:
    print(f"{r['task']}: {r['output']} ({r['duration_ms']}ms)")
```

---

## 🔥 Performance Tips

1. **Use Pipeline for Multiple Operations**
   - Pipeline batches operations efficiently
   - Better than calling functions individually

2. **Adjust Verbose Level**
   - Use level 0 for production
   - Use level 3 only for debugging

3. **Parallel Scanning**
   - Port scanning uses parallel processing automatically
   - No need for manual threading

4. **Cache Results**
   - Cache system info if polling frequently
   - Don't refresh every iteration

---

## ⚠️ Important Notes

### Network Functions
- Requires active internet connection
- May be blocked by firewalls
- Respects DNS resolution

### Security Scanning
- **ALWAYS get permission before scanning**
- Only scan systems you own or have permission to test
- False positives are possible
- Results are indicators, not definitive proof

### System Functions
- Some functions require root/admin on certain systems
- Termux may have limited access to some system info
- Performance varies by device

---

## 📚 References

- [PyO3 Documentation](https://pyo3.rs/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [OWASP Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)

---

**Last Updated:** 2024
**Version:** 0.1.0

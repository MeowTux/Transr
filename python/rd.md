# TransR Quick Reference

Quick reference guide untuk menggunakan TransR di Python.

## 📦 Installation

```bash
maturin develop --release
```

## 🚀 Quick Start

```python
from transR import PipelineR, Feature

# Pipeline
p = PipelineR()
p.set_verbose(2)
p.print("Hello!")
p.system()
p.run()

# Direct feature access
f = Feature()
info = f.system_info()
print(info['os_name'])
```

## 🔄 Pipeline Methods

| Method | Description | Example |
|--------|-------------|---------|
| `set_verbose(level)` | Set verbose (0-3) | `p.set_verbose(2)` |
| `set_debug(bool)` | Enable debug mode | `p.set_debug(True)` |
| `print(text)` | Print text | `p.print("Hello")` |
| `system()` | Check system | `p.system()` |
| `time()` | Check time | `p.time()` |
| `math(op, args)` | Math operation | `p.math("sum", [1,2,3])` |
| `http_get(url)` | HTTP GET | `p.http_get("https://...")` |
| `hash(algo, data)` | Hash data | `p.hash("sha256", "text")` |
| `loop_calc(n)` | Heavy loop | `p.loop_calc(10000)` |
| `blockchain(h, w)` | Blockchain sim | `p.blockchain(100, 100)` |
| `scan_ports(host, ports)` | Port scan | `p.scan_ports("localhost", [80,443])` |
| `scan_vulns(url)` | Vuln scan | `p.scan_vulns("http://...")` |
| `run()` | Execute pipeline | `p.run()` |
| `clear()` | Clear tasks | `p.clear()` |
| `results()` | Get results | `r = p.results()` |

## 🖥️ System Features

```python
f = Feature()

# System info
info = f.system_info()
# Keys: os_name, os_version, kernel, hostname, cpu_count, 
#       cpu_brand, total_memory, used_memory, uptime

# CPU & Memory
cpu = f.cpu_usage()  # List of CPU usage per core
mem = f.memory_percent()  # Memory usage percentage
```

## ⏰ Time Features

```python
# Current time
now = f.now()
# Keys: year, month, day, hour, minute, second, 
#       weekday, formatted, time_of_day

# Timestamp
ts = f.timestamp()  # Unix timestamp

# Greeting
greeting = f.greeting()  # Time-based greeting
```

## 🔢 Math Features

```python
# Statistics
f.sum([1,2,3,4,5])  # 15.0
f.mean([10,20,30])  # 20.0

# Basic math
f.sqrt(16)  # 4.0

# Number theory
f.factorial(5)  # "120"
f.is_prime(17)  # True

# Random
f.random()  # 0.0 to 1.0
f.random_range(1, 100)  # Random int 1-100
```

## 🔐 Crypto Features

```python
# Hashing
f.hash_sha256("data")  # SHA256 hash
f.hash_md5("data")  # MD5 hash
```

## 🌐 Network Features

```python
# HTTP GET
response = f.http_get("https://api.github.com")
# Returns: {'status': 200, 'body': '...', 'url': '...'}
```

## 🔍 Security Features

```python
# Port scanning
result = f.scan_port("127.0.0.1", 80)
# Returns: {'host': '...', 'port': 80, 'is_open': bool, 'service': '...'}

scan = f.quick_scan("scanme.nmap.org")
# Returns: {'host': '...', 'is_alive': bool, 'open_ports': [...], 'services': [...]}

# Vulnerability scanning
vulns = f.vuln_scan("http://target.com")
# Returns: [{'id': '...', 'name': '...', 'severity': '...', 'url': '...', 'evidence': '...'}]
```

## 📊 Examples

### System Monitor
```python
from transR import Feature
import time

f = Feature()
while True:
    cpu = f.cpu_usage()
    mem = f.memory_percent()
    print(f"CPU: {sum(cpu)/len(cpu):.1f}% | RAM: {mem:.1f}%")
    time.sleep(1)
```

### Network Scanner
```python
f = Feature()
targets = ["192.168.1.1", "192.168.1.100"]

for host in targets:
    result = f.quick_scan(host)
    if result['is_alive']:
        print(f"{host}: {result['open_ports']}")
```

### Security Audit
```python
f = Feature()
vulns = f.vuln_scan("http://target.com")

for v in vulns:
    if v['severity'] in ['Critical', 'High']:
        print(f"[{v['severity']}] {v['name']}")
```

### Data Pipeline
```python
p = PipelineR()
p.set_verbose(1)

p.http_get("https://api.example.com/data")
p.math("mean", [10, 20, 30])
p.hash("sha256", "result")
p.run()

results = p.results()
for r in results:
    print(f"{r['task']}: {r['duration_ms']}ms")
```

### Blockchain Simulation
```python
p = PipelineR()
p.set_verbose(2)

# 100x100 = 10,000 iterations with random ops
p.blockchain(100, 100)
p.run()
```

## ⚙️ Verbose Levels

| Level | Name | Description |
|-------|------|-------------|
| 0 | Silent | No output |
| 1 | Info | Basic info |
| 2 | Debug | Detailed info |
| 3 | Trace | Very detailed |

## ⚠️ Notes

- Network features require internet connection
- Security scanning requires permission
- Some system features may need elevated privileges
- Performance varies by device

## 🔗 Links

- Full docs: `README.md`
- Function docs: `DOC-FUNC.md`
- Tests: `python/test/`

# TransR - Advanced Rust Extension for Python

🚀 **TransR** adalah library Rust yang powerful dengan Python bindings menggunakan PyO3. Dirancang khusus untuk Termux dan sistem Linux, TransR menyediakan berbagai fitur advanced mulai dari system monitoring, cryptography, network scanning, hingga vulnerability assessment.

## ✨ Features

### 🔧 Core Features
- **Pipeline System**: Sistem pipeline yang fleksibel untuk menjalankan multiple tasks
- **Verbose & Debug Mode**: 3-level verbose logging dan debug mode
- **Error Handling**: Human-friendly error messages menggunakan thiserror
- **Decorator System**: Python-like decorators untuk task management

### 📊 System Functions (SysEz)
- System information (OS, CPU, Memory, Uptime)
- CPU usage monitoring
- Memory usage tracking
- Disk information
- Process monitoring

### ⏰ Time Functions (TimerU & NowTime)
- Current time & date
- Timestamp operations
- Time formatting
- Timer/Benchmark utilities
- Greeting berdasarkan waktu

### 🔢 Math Functions (RMath)
- Basic operations (add, sub, mul, div, pow, sqrt)
- Trigonometric functions (sin, cos, tan)
- Statistical functions (mean, median, variance, std_dev)
- Number theory (factorial, gcd, lcm, is_prime, fibonacci)
- Random number generation

### 🌐 Network Functions (RNet)
- HTTP requests (GET, POST, PUT, DELETE)
- File download
- Ping test
- Custom headers support

### 🔐 Cryptography
- SHA256 hashing
- MD5 hashing
- More algorithms coming soon

### 🔍 Security Tools

#### RustNetX (Port Scanner)
- Single port scanning
- Multiple port scanning
- Port range scanning
- Quick scan (common ports)
- Full scan (1-1024)
- Service identification
- Banner grabbing

#### HyrOS (Vulnerability Scanner)
- Directory traversal detection
- SQL injection detection
- XSS vulnerability detection
- Open redirect detection
- Exposed files detection (.git, .env)
- Debug mode detection
- Missing security headers
- API exposure check

## 📦 Installation

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python development headers
pkg install python python-dev

# Install maturin (PyO3 build tool)
pip install maturin
```

### Build & Install
```bash
# Clone repository
git clone <repo-url>
cd transR-project

# Build dengan maturin
maturin develop --release

# Atau build wheel
maturin build --release
pip install target/wheels/*.whl
```

## 🚀 Quick Start

### Basic Pipeline Example
```python
from transR import PipelineR

# Create pipeline
p = PipelineR()

# Set verbose level (0=silent, 1=info, 2=debug, 3=trace)
p.set_verbose(2)

# Add tasks to pipeline
p.print("Hello from TransR!")
p.system()
p.time()
p.hash("sha256", "password123")

# Run pipeline
p.run()

# Get results
results = p.results()
for r in results:
    print(f"{r['task']}: {r['output']} ({r['duration_ms']}ms)")
```

### Advanced Pipeline Example
```python
from transR import PipelineR

p = PipelineR()
p.set_verbose(3)
p.set_debug(True)  # Enable debug mode

# Math operations
p.math("sum", [1, 2, 3, 4, 5])
p.math("mean", [10, 20, 30, 40, 50])

# Network operations
p.http_get("https://api.github.com")

# Blockchain-style loop (height × weight iterations)
p.blockchain(100, 100)  # 10,000 iterations with random operations

# Security scans
p.scan_ports("127.0.0.1", [80, 443, 8080])
p.scan_vulns("http://example.com")

p.run()
```

### Feature Module Example
```python
from transR import Feature

f = Feature()

# System info
info = f.system_info()
print(f"OS: {info['os_name']} {info['os_version']}")
print(f"CPU: {info['cpu_brand']} ({info['cpu_count']} cores)")

cpu_usage = f.cpu_usage()
print(f"CPU Usage: {cpu_usage}")

mem_percent = f.memory_percent()
print(f"Memory: {mem_percent:.2f}%")

# Time info
now = f.now()
print(f"Time: {now['formatted']}")
print(f"Period: {now['time_of_day']}")
print(f.greeting())  # Good Morning/Afternoon/Evening/Night

# Math operations
print(f"Sum: {f.sum([1, 2, 3, 4, 5])}")
print(f"Mean: {f.mean([10, 20, 30])}")
print(f"Factorial(10): {f.factorial(10)}")
print(f"Is 17 prime? {f.is_prime(17)}")
print(f"Random: {f.random()}")
print(f"Random range: {f.random_range(1, 100)}")

# Network
response = f.http_get("https://httpbin.org/get")
print(f"Status: {response['status']}")

# Hashing
print(f"SHA256: {f.hash_sha256('hello')}")
print(f"MD5: {f.hash_md5('hello')}")

# Port scanning
result = f.scan_port("127.0.0.1", 80)
print(f"Port {result['port']}: {'OPEN' if result['is_open'] else 'CLOSED'}")

scan = f.quick_scan("scanme.nmap.org")
print(f"Host alive: {scan['is_alive']}")
print(f"Open ports: {scan['open_ports']}")

# Vulnerability scanning
vulns = f.vuln_scan("http://testphp.vulnweb.com")
for v in vulns:
    print(f"[{v['severity']}] {v['name']}")
```

## 📚 Documentation

Lihat file berikut untuk dokumentasi detail:
- `DOC-FUNC.md` - Dokumentasi lengkap untuk functions dan plugins
- `python/rd.md` - Quick reference untuk Python testing

## 🧪 Testing

```bash
cd python

# Test basic features
python test/test.py

# Test functions
python test/MyFunc.py

# Test plugins
python test/MyPlugins.py

# Test all
python test/TestAll.py

# Unit testing
python Unit/Unitesting.py
```

## 🏗️ Project Structure

```
transR-project/
├── Cargo.toml              # Rust dependencies
├── README.md               # This file
├── DOC-FUNC.md            # Function documentation
│
├── src/
│   ├── lib.rs             # PyO3 main entry
│   ├── errhuman.rs        # Error handling
│   ├── vvv.rs             # Verbose logging
│   ├── debug.rs           # Debug mode
│   ├── transr.rs          # Pipeline system
│   │
│   ├── func/              # Core functions
│   │   ├── sysez.rs       # System operations
│   │   ├── timeru.rs      # Time operations
│   │   ├── nowtime.rs     # DateTime operations
│   │   ├── rmath.rs       # Math operations
│   │   └── rnet.rs        # Network operations
│   │
│   └── plugins/           # Advanced plugins
│       ├── decorator.rs   # Decorator system
│       ├── rustnetx.rs    # Port scanner
│       └── hyros.rs       # Vulnerability scanner
│
└── python/
    ├── test/              # Test files
    │   ├── test.py
    │   ├── MyFunc.py
    │   ├── MyPlugins.py
    │   └── TestAll.py
    │
    ├── Unit/              # Unit tests
    │   ├── Unit1.py
    │   ├── Unitesting.py
    │   └── debuggingUnit.py
    │
    └── rd.md              # Quick reference
```

## 🎯 Use Cases

### 1. System Monitoring
```python
from transR import Feature
f = Feature()

while True:
    cpu = f.cpu_usage()
    mem = f.memory_percent()
    print(f"CPU: {sum(cpu)/len(cpu):.1f}% | RAM: {mem:.1f}%")
    time.sleep(1)
```

### 2. Security Auditing
```python
from transR import Feature
f = Feature()

# Scan network
hosts = ["192.168.1.1", "192.168.1.100"]
for host in hosts:
    result = f.quick_scan(host)
    if result['is_alive']:
        print(f"{host}: {result['open_ports']}")

# Check vulnerabilities
vulns = f.vuln_scan("http://target.com")
for v in vulns:
    if v['severity'] in ['Critical', 'High']:
        print(f"⚠️  {v['name']}")
```

### 3. Data Processing Pipeline
```python
from transR import PipelineR

p = PipelineR()
p.http_get("https://api.example.com/data")
p.math("mean", data)
p.hash("sha256", result)
p.run()
```

## ⚙️ Configuration

### Verbose Levels
- `0` - Silent (no output)
- `1` - Info (basic information)
- `2` - Debug (detailed information)
- `3` - Trace (very detailed, includes all operations)

### Debug Mode
Enable debug mode untuk detailed execution information dan benchmarking.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests.

## 📝 License

MIT License

## 🙏 Credits

Built with:
- [PyO3](https://pyo3.rs/) - Rust bindings for Python
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information
- [chrono](https://github.com/chronotope/chrono) - Date and time
- [sha2](https://github.com/RustCrypto/hashes) - Cryptographic hashing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [rayon](https://github.com/rayon-rs/rayon) - Parallel processing

---

Made with ❤️ using Rust + Python

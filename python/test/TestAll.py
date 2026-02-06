#!/usr/bin/env python3
"""
TransR - Comprehensive Test Suite
Test all features, functions, and plugins
"""

import sys
import time

try:
    from transR import PipelineR, Feature
    print("✓ TransR module loaded successfully\n")
except ImportError as e:
    print(f"✗ Failed to import transR: {e}")
    sys.exit(1)

class TestRunner:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.skipped = 0
        self.results = []
    
    def run_test(self, name, func, *args, **kwargs):
        """Run a single test"""
        print(f"\n{'='*60}")
        print(f"TEST: {name}")
        print('='*60)
        
        try:
            start = time.time()
            result = func(*args, **kwargs)
            elapsed = time.time() - start
            
            if result:
                self.passed += 1
                status = "✓ PASS"
                print(f"\n{status} ({elapsed:.3f}s)")
            else:
                self.failed += 1
                status = "✗ FAIL"
                print(f"\n{status} ({elapsed:.3f}s)")
            
            self.results.append((name, status, elapsed))
            return result
            
        except Exception as e:
            self.failed += 1
            status = "✗ ERROR"
            print(f"\n{status}: {e}")
            self.results.append((name, status, 0))
            return False
    
    def print_summary(self):
        """Print test summary"""
        total = self.passed + self.failed + self.skipped
        
        print("\n" + "="*60)
        print("TEST SUMMARY")
        print("="*60)
        
        for name, status, elapsed in self.results:
            if elapsed > 0:
                print(f"{status}: {name} ({elapsed:.3f}s)")
            else:
                print(f"{status}: {name}")
        
        print("\n" + "-"*60)
        print(f"Total:   {total}")
        print(f"Passed:  {self.passed} ({self.passed/total*100:.1f}%)")
        print(f"Failed:  {self.failed}")
        print(f"Skipped: {self.skipped}")
        print("="*60)

# Test Functions
def test_basic_pipeline():
    """Test basic pipeline functionality"""
    p = PipelineR()
    p.set_verbose(1)
    p.print("Pipeline test")
    p.run()
    return True

def test_system_monitoring():
    """Test system monitoring features"""
    f = Feature()
    
    info = f.system_info()
    cpu = f.cpu_usage()
    mem = f.memory_percent()
    
    print(f"OS: {info['os_name']}")
    print(f"CPU Cores: {info['cpu_count']}")
    print(f"CPU Usage: {sum(cpu)/len(cpu):.1f}%")
    print(f"Memory: {mem:.1f}%")
    
    return all([info, cpu, mem is not None])

def test_time_features():
    """Test time and date features"""
    f = Feature()
    
    now = f.now()
    ts = f.timestamp()
    greeting = f.greeting()
    
    print(f"Time: {now['formatted']}")
    print(f"Timestamp: {ts}")
    print(f"Greeting: {greeting}")
    
    return all([now, ts, greeting])

def test_math_operations():
    """Test mathematical operations"""
    f = Feature()
    
    numbers = [10, 20, 30, 40, 50]
    
    sum_result = f.sum(numbers)
    mean_result = f.mean(numbers)
    sqrt_result = f.sqrt(25)
    factorial_result = f.factorial(5)
    is_prime_result = f.is_prime(17)
    random_result = f.random()
    
    print(f"Sum: {sum_result}")
    print(f"Mean: {mean_result}")
    print(f"sqrt(25): {sqrt_result}")
    print(f"5!: {factorial_result}")
    print(f"is_prime(17): {is_prime_result}")
    print(f"random: {random_result}")
    
    return all([
        sum_result == 150,
        mean_result == 30,
        sqrt_result == 5.0,
        is_prime_result == True,
        0 <= random_result <= 1
    ])

def test_cryptography():
    """Test cryptographic functions"""
    f = Feature()
    
    data = "test_data"
    sha256 = f.hash_sha256(data)
    md5 = f.hash_md5(data)
    
    print(f"Data: {data}")
    print(f"SHA256: {sha256}")
    print(f"MD5: {md5}")
    
    return all([
        len(sha256) == 64,
        len(md5) == 32
    ])

def test_network_basic():
    """Test basic network functionality"""
    f = Feature()
    
    try:
        response = f.http_get("https://httpbin.org/get")
        print(f"Status: {response['status']}")
        print(f"Body length: {len(response['body'])} bytes")
        return response['status'] == 200
    except Exception as e:
        print(f"Network test skipped (no internet): {e}")
        return None  # Skipped

def test_port_scanning():
    """Test port scanning"""
    f = Feature()
    
    try:
        # Scan localhost common ports
        result = f.scan_port("127.0.0.1", 80)
        print(f"Port 80: {'OPEN' if result['is_open'] else 'CLOSED'}")
        
        return True  # Test successful even if port is closed
    except Exception as e:
        print(f"Port scan error: {e}")
        return False

def test_vulnerability_scan():
    """Test vulnerability scanning"""
    f = Feature()
    
    try:
        # Note: This requires internet connection
        results = f.vuln_scan("http://testphp.vulnweb.com")
        
        if results:
            print(f"Found {len(results)} potential vulnerabilities")
            critical = sum(1 for v in results if v['severity'] == 'Critical')
            high = sum(1 for v in results if v['severity'] == 'High')
            print(f"Critical: {critical}, High: {high}")
            return True
        else:
            print("No vulnerabilities detected or target unavailable")
            return None  # Skipped
    except Exception as e:
        print(f"Vuln scan skipped (no internet): {e}")
        return None  # Skipped

def test_pipeline_complex():
    """Test complex pipeline with multiple operations"""
    p = PipelineR()
    p.set_verbose(0)  # Silent
    
    # Add various tasks
    p.print("Complex pipeline test")
    p.system()
    p.time()
    p.math("sum", [1, 2, 3, 4, 5])
    p.hash("sha256", "test")
    p.loop_calc(10000)
    
    p.run()
    results = p.results()
    
    print(f"Executed {len(results)} tasks")
    total_time = sum(r['duration_ms'] for r in results)
    print(f"Total time: {total_time}ms")
    
    return len(results) == 6

def test_performance_benchmark():
    """Test performance with heavy operations"""
    p = PipelineR()
    p.set_verbose(0)
    
    iterations = 100000
    print(f"Running {iterations:,} iterations...")
    
    start = time.time()
    p.loop_calc(iterations)
    p.run()
    elapsed = time.time() - start
    
    rate = iterations / elapsed
    print(f"Time: {elapsed:.3f}s")
    print(f"Rate: {rate:,.0f} ops/sec")
    
    return elapsed < 10  # Should complete in under 10 seconds

def test_blockchain_simulation():
    """Test blockchain-style random calculations"""
    p = PipelineR()
    p.set_verbose(0)
    
    height = 50
    weight = 50
    total = height * weight
    
    print(f"Blockchain simulation: {height}x{weight} = {total:,} iterations")
    
    start = time.time()
    p.blockchain(height, weight)
    p.run()
    elapsed = time.time() - start
    
    print(f"Time: {elapsed:.3f}s")
    
    return elapsed < 5

def test_feature_integration():
    """Test integration of multiple features"""
    f = Feature()
    p = PipelineR()
    
    # Use Feature
    info = f.system_info()
    now = f.now()
    
    # Use Pipeline
    p.print(f"Integration test: {now['formatted']}")
    p.system()
    p.run()
    
    print(f"System: {info['os_name']}")
    print(f"Time: {now['formatted']}")
    
    return True

def test_error_handling():
    """Test error handling"""
    f = Feature()
    
    try:
        # This should fail gracefully
        f.mean([])  # Empty list
        return False  # Should have raised error
    except:
        print("✓ Error handled correctly for invalid input")
        return True

def main():
    """Run all tests"""
    print("="*60)
    print("TransR - COMPREHENSIVE TEST SUITE")
    print("="*60)
    print("\nThis will test all features, functions, and plugins.")
    print("Some tests require internet connection and may be skipped.\n")
    
    runner = TestRunner()
    
    # Core Tests
    print("\n" + "="*60)
    print("CORE FUNCTIONALITY TESTS")
    print("="*60)
    
    runner.run_test("Basic Pipeline", test_basic_pipeline)
    runner.run_test("System Monitoring", test_system_monitoring)
    runner.run_test("Time Features", test_time_features)
    runner.run_test("Math Operations", test_math_operations)
    runner.run_test("Cryptography", test_cryptography)
    
    # Network Tests
    print("\n" + "="*60)
    print("NETWORK TESTS (may be skipped)")
    print("="*60)
    
    result = runner.run_test("Basic Network", test_network_basic)
    if result is None:
        runner.skipped += 1
        runner.passed -= 1
    
    # Security Tests
    print("\n" + "="*60)
    print("SECURITY PLUGIN TESTS")
    print("="*60)
    
    runner.run_test("Port Scanning", test_port_scanning)
    
    result = runner.run_test("Vulnerability Scanning", test_vulnerability_scan)
    if result is None:
        runner.skipped += 1
        runner.passed -= 1
    
    # Advanced Tests
    print("\n" + "="*60)
    print("ADVANCED TESTS")
    print("="*60)
    
    runner.run_test("Complex Pipeline", test_pipeline_complex)
    runner.run_test("Performance Benchmark", test_performance_benchmark)
    runner.run_test("Blockchain Simulation", test_blockchain_simulation)
    runner.run_test("Feature Integration", test_feature_integration)
    runner.run_test("Error Handling", test_error_handling)
    
    # Print summary
    runner.print_summary()
    
    # Return exit code
    return 0 if runner.failed == 0 else 1

if __name__ == "__main__":
    sys.exit(main())

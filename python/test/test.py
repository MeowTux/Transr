#!/usr/bin/env python3
"""
TransR - Main Test File
Testing basic functionality
"""

import sys
import time

try:
    from transR import PipelineR, Feature
    print("✓ TransR module imported successfully")
except ImportError as e:
    print(f"✗ Failed to import transR: {e}")
    print("Make sure to build the module first:")
    print("  maturin develop --release")
    sys.exit(1)

def test_pipeline_basic():
    """Test basic pipeline operations"""
    print("\n" + "="*60)
    print("TEST 1: Basic Pipeline")
    print("="*60)
    
    p = PipelineR()
    p.set_verbose(2)
    
    p.print("Hello from TransR!")
    p.print("Testing pipeline system...")
    
    try:
        p.run()
        print("✓ Basic pipeline test passed")
        return True
    except Exception as e:
        print(f"✗ Basic pipeline test failed: {e}")
        return False

def test_system_info():
    """Test system information"""
    print("\n" + "="*60)
    print("TEST 2: System Information")
    print("="*60)
    
    try:
        f = Feature()
        info = f.system_info()
        
        print(f"OS: {info['os_name']} {info['os_version']}")
        print(f"Kernel: {info['kernel']}")
        print(f"Hostname: {info['hostname']}")
        print(f"CPU: {info['cpu_brand']}")
        print(f"Cores: {info['cpu_count']}")
        print(f"RAM: {info['used_memory'] / 1024**3:.2f}GB / {info['total_memory'] / 1024**3:.2f}GB")
        print(f"Uptime: {info['uptime']} seconds")
        
        print("✓ System info test passed")
        return True
    except Exception as e:
        print(f"✗ System info test failed: {e}")
        return False

def test_time_operations():
    """Test time operations"""
    print("\n" + "="*60)
    print("TEST 3: Time Operations")
    print("="*60)
    
    try:
        f = Feature()
        
        # Current time
        now = f.now()
        print(f"Date: {now['year']}-{now['month']:02d}-{now['day']:02d}")
        print(f"Time: {now['hour']:02d}:{now['minute']:02d}:{now['second']:02d}")
        print(f"Weekday: {now['weekday']}")
        print(f"Period: {now['time_of_day']}")
        
        # Timestamp
        ts = f.timestamp()
        print(f"Timestamp: {ts}")
        
        # Greeting
        greeting = f.greeting()
        print(f"Greeting: {greeting}")
        
        print("✓ Time operations test passed")
        return True
    except Exception as e:
        print(f"✗ Time operations test failed: {e}")
        return False

def test_math_operations():
    """Test math operations"""
    print("\n" + "="*60)
    print("TEST 4: Math Operations")
    print("="*60)
    
    try:
        f = Feature()
        
        # Basic operations
        numbers = [1, 2, 3, 4, 5]
        print(f"Numbers: {numbers}")
        print(f"Sum: {f.sum(numbers)}")
        print(f"Mean: {f.mean(numbers)}")
        
        # Advanced operations
        print(f"sqrt(16): {f.sqrt(16)}")
        print(f"factorial(5): {f.factorial(5)}")
        print(f"is_prime(17): {f.is_prime(17)}")
        print(f"is_prime(18): {f.is_prime(18)}")
        
        # Random
        print(f"random(): {f.random()}")
        print(f"random_range(1, 100): {f.random_range(1, 100)}")
        
        print("✓ Math operations test passed")
        return True
    except Exception as e:
        print(f"✗ Math operations test failed: {e}")
        return False

def test_hashing():
    """Test cryptographic hashing"""
    print("\n" + "="*60)
    print("TEST 5: Cryptographic Hashing")
    print("="*60)
    
    try:
        f = Feature()
        
        data = "Hello, TransR!"
        print(f"Data: {data}")
        
        sha256 = f.hash_sha256(data)
        print(f"SHA256: {sha256}")
        
        md5 = f.hash_md5(data)
        print(f"MD5: {md5}")
        
        print("✓ Hashing test passed")
        return True
    except Exception as e:
        print(f"✗ Hashing test failed: {e}")
        return False

def test_pipeline_advanced():
    """Test advanced pipeline with multiple operations"""
    print("\n" + "="*60)
    print("TEST 6: Advanced Pipeline")
    print("="*60)
    
    try:
        p = PipelineR()
        p.set_verbose(1)
        
        # Add multiple tasks
        p.print("Starting advanced pipeline...")
        p.system()
        p.time()
        p.math("sum", [10, 20, 30, 40, 50])
        p.hash("sha256", "test_data")
        p.loop_calc(10000)
        
        # Run pipeline
        print("\nExecuting pipeline...")
        p.run()
        
        # Get results
        results = p.results()
        print(f"\nTotal tasks executed: {len(results)}")
        
        total_time = sum(r['duration_ms'] for r in results)
        print(f"Total execution time: {total_time}ms")
        
        print("✓ Advanced pipeline test passed")
        return True
    except Exception as e:
        print(f"✗ Advanced pipeline test failed: {e}")
        return False

def test_performance():
    """Test performance with heavy operations"""
    print("\n" + "="*60)
    print("TEST 7: Performance Test")
    print("="*60)
    
    try:
        p = PipelineR()
        p.set_verbose(0)  # Silent mode for performance
        
        # Heavy loop
        iterations = 1000000
        print(f"Running {iterations:,} iterations...")
        
        start = time.time()
        p.loop_calc(iterations)
        p.run()
        elapsed = time.time() - start
        
        print(f"Time: {elapsed:.3f}s")
        print(f"Rate: {iterations/elapsed:,.0f} ops/sec")
        
        # Blockchain simulation
        print("\nBlockchain simulation (100x100)...")
        p.clear()
        
        start = time.time()
        p.blockchain(100, 100)
        p.run()
        elapsed = time.time() - start
        
        print(f"Time: {elapsed:.3f}s")
        
        print("✓ Performance test passed")
        return True
    except Exception as e:
        print(f"✗ Performance test failed: {e}")
        return False

def main():
    """Run all tests"""
    print("="*60)
    print("TransR Test Suite")
    print("="*60)
    
    tests = [
        ("Basic Pipeline", test_pipeline_basic),
        ("System Information", test_system_info),
        ("Time Operations", test_time_operations),
        ("Math Operations", test_math_operations),
        ("Cryptographic Hashing", test_hashing),
        ("Advanced Pipeline", test_pipeline_advanced),
        ("Performance", test_performance),
    ]
    
    results = []
    
    for name, test_func in tests:
        try:
            result = test_func()
            results.append((name, result))
        except Exception as e:
            print(f"\n✗ Test '{name}' crashed: {e}")
            results.append((name, False))
    
    # Summary
    print("\n" + "="*60)
    print("TEST SUMMARY")
    print("="*60)
    
    passed = sum(1 for _, result in results if result)
    total = len(results)
    
    for name, result in results:
        status = "✓ PASS" if result else "✗ FAIL"
        print(f"{status}: {name}")
    
    print(f"\nTotal: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed!")
        return 0
    else:
        print(f"⚠️  {total - passed} test(s) failed")
        return 1

if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""
TransR - Function Testing
Testing all feature functions
"""

import sys

try:
    from transR import Feature
except ImportError:
    print("✗ Failed to import transR")
    sys.exit(1)

def separator(title):
    print("\n" + "="*60)
    print(f" {title}")
    print("="*60)

def test_system_functions():
    separator("SYSTEM FUNCTIONS")
    
    f = Feature()
    
    # System info
    print("\n1. System Information:")
    info = f.system_info()
    for key, value in info.items():
        if 'memory' in key:
            print(f"  {key}: {value / 1024**3:.2f} GB")
        else:
            print(f"  {key}: {value}")
    
    # CPU usage
    print("\n2. CPU Usage:")
    cpu = f.cpu_usage()
    for i, usage in enumerate(cpu):
        print(f"  Core {i}: {usage:.1f}%")
    print(f"  Average: {sum(cpu)/len(cpu):.1f}%")
    
    # Memory
    print("\n3. Memory Usage:")
    mem = f.memory_percent()
    print(f"  Memory: {mem:.2f}%")

def test_time_functions():
    separator("TIME FUNCTIONS")
    
    f = Feature()
    
    # Current time
    print("\n1. Current Time:")
    now = f.now()
    print(f"  Full: {now['formatted']}")
    print(f"  Date: {now['year']}-{now['month']:02d}-{now['day']:02d}")
    print(f"  Time: {now['hour']:02d}:{now['minute']:02d}:{now['second']:02d}")
    print(f"  Weekday: {now['weekday']}")
    print(f"  Period: {now['time_of_day']}")
    print(f"  ISO Week: {now['iso_week']}")
    
    # Timestamp
    print("\n2. Timestamp:")
    ts = f.timestamp()
    print(f"  Unix timestamp: {ts}")
    
    # Greeting
    print("\n3. Time-based Greeting:")
    greeting = f.greeting()
    print(f"  {greeting}")

def test_math_functions():
    separator("MATH FUNCTIONS")
    
    f = Feature()
    
    # Test data
    numbers = [10, 20, 30, 40, 50]
    
    # Statistics
    print("\n1. Statistical Functions:")
    print(f"  Numbers: {numbers}")
    print(f"  Sum: {f.sum(numbers)}")
    print(f"  Mean: {f.mean(numbers)}")
    
    # Basic math
    print("\n2. Basic Operations:")
    print(f"  sqrt(16): {f.sqrt(16)}")
    print(f"  sqrt(25): {f.sqrt(25)}")
    print(f"  sqrt(100): {f.sqrt(100)}")
    
    # Number theory
    print("\n3. Number Theory:")
    for n in [5, 10, 15, 20]:
        print(f"  factorial({n}): {f.factorial(n)}")
    
    print("\n4. Prime Numbers:")
    test_nums = [2, 3, 4, 5, 11, 15, 17, 19, 20, 23]
    for n in test_nums:
        is_prime = f.is_prime(n)
        status = "✓ PRIME" if is_prime else "  composite"
        print(f"  {n:3d}: {status}")
    
    # Random
    print("\n5. Random Numbers:")
    print(f"  random(): {f.random()}")
    print(f"  random(): {f.random()}")
    print(f"  random(): {f.random()}")
    print(f"  random_range(1, 100): {f.random_range(1, 100)}")
    print(f"  random_range(1, 100): {f.random_range(1, 100)}")
    print(f"  random_range(1, 100): {f.random_range(1, 100)}")

def test_crypto_functions():
    separator("CRYPTOGRAPHY FUNCTIONS")
    
    f = Feature()
    
    # Test data
    test_strings = [
        "hello",
        "password123",
        "TransR is awesome!",
        "The quick brown fox jumps over the lazy dog",
    ]
    
    print("\n1. SHA256 Hashing:")
    for s in test_strings:
        hash_val = f.hash_sha256(s)
        print(f"  '{s[:30]}...' if len(s) > 30 else '{s}'")
        print(f"    SHA256: {hash_val}")
    
    print("\n2. MD5 Hashing:")
    for s in test_strings:
        hash_val = f.hash_md5(s)
        print(f"  '{s[:30]}...' if len(s) > 30 else '{s}'")
        print(f"    MD5: {hash_val}")

def test_network_functions():
    separator("NETWORK FUNCTIONS")
    
    f = Feature()
    
    print("\n1. HTTP GET Request:")
    try:
        response = f.http_get("https://httpbin.org/get")
        print(f"  Status: {response['status']}")
        print(f"  URL: {response['url']}")
        print(f"  Body length: {len(response['body'])} bytes")
        
        if response['status'] == 200:
            print("  ✓ Request successful")
        else:
            print(f"  ⚠️  Unexpected status: {response['status']}")
    except Exception as e:
        print(f"  ✗ Request failed: {e}")
        print("  (This is expected if no internet connection)")

def main():
    print("="*60)
    print(" TransR Feature Functions Test Suite")
    print("="*60)
    
    try:
        test_system_functions()
        test_time_functions()
        test_math_functions()
        test_crypto_functions()
        test_network_functions()
        
        print("\n" + "="*60)
        print(" ✓ All function tests completed")
        print("="*60)
        
        return 0
    except Exception as e:
        print(f"\n✗ Test failed with error: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())

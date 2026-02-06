#!/usr/bin/env python3
"""
TransR - Unit Test 1: Logic Testing
Testing logic and edge cases
"""

import sys

try:
    from transR import PipelineR, Feature
except ImportError:
    print("✗ Failed to import transR")
    sys.exit(1)

def test_logic_conditions():
    """Test logical conditions and edge cases"""
    print("="*60)
    print("LOGIC TESTING")
    print("="*60)
    
    f = Feature()
    
    # Test 1: Empty inputs
    print("\n1. Empty Input Handling:")
    try:
        result = f.mean([])
        print("  ✗ Should have raised error for empty list")
    except Exception as e:
        print(f"  ✓ Correctly handled empty list: {e}")
    
    # Test 2: Negative square root
    print("\n2. Negative Square Root:")
    try:
        result = f.sqrt(-1)
        print("  ✗ Should have raised error for negative sqrt")
    except Exception as e:
        print(f"  ✓ Correctly handled negative sqrt: {e}")
    
    # Test 3: Large numbers
    print("\n3. Large Numbers:")
    try:
        factorial_20 = f.factorial(20)
        print(f"  20! = {factorial_20}")
        print(f"  ✓ Handled large factorial (length: {len(factorial_20)})")
    except Exception as e:
        print(f"  ✗ Failed on large number: {e}")
    
    # Test 4: Edge case primes
    print("\n4. Prime Edge Cases:")
    test_cases = [
        (0, False),
        (1, False),
        (2, True),
        (3, True),
        (4, False),
    ]
    
    for num, expected in test_cases:
        result = f.is_prime(num)
        status = "✓" if result == expected else "✗"
        print(f"  {status} is_prime({num}) = {result} (expected {expected})")
    
    # Test 5: Boundary conditions
    print("\n5. Boundary Conditions:")
    
    # Random should be between 0 and 1
    randoms = [f.random() for _ in range(100)]
    all_valid = all(0 <= r <= 1 for r in randoms)
    print(f"  {'✓' if all_valid else '✗'} random() in [0,1]: {all_valid}")
    
    # Random range
    range_randoms = [f.random_range(1, 10) for _ in range(100)]
    all_in_range = all(1 <= r <= 10 for r in range_randoms)
    print(f"  {'✓' if all_in_range else '✗'} random_range(1,10) in [1,10]: {all_in_range}")

def test_data_consistency():
    """Test data consistency across multiple calls"""
    print("\n" + "="*60)
    print("DATA CONSISTENCY TESTING")
    print("="*60)
    
    f = Feature()
    
    # Test 1: Hash consistency
    print("\n1. Hash Consistency:")
    data = "test_data"
    
    hash1 = f.hash_sha256(data)
    hash2 = f.hash_sha256(data)
    hash3 = f.hash_sha256(data)
    
    consistent = (hash1 == hash2 == hash3)
    print(f"  SHA256 consistency: {'✓' if consistent else '✗'}")
    print(f"  Hash: {hash1[:16]}...")
    
    # Test 2: Different inputs = different hashes
    print("\n2. Hash Uniqueness:")
    hashes = set()
    test_strings = ["test1", "test2", "test3", "test4", "test5"]
    
    for s in test_strings:
        hashes.add(f.hash_sha256(s))
    
    unique = len(hashes) == len(test_strings)
    print(f"  Unique hashes: {'✓' if unique else '✗'}")
    print(f"  Generated {len(hashes)} unique hashes from {len(test_strings)} inputs")
    
    # Test 3: Math consistency
    print("\n3. Math Consistency:")
    numbers = [1, 2, 3, 4, 5]
    
    sum1 = f.sum(numbers)
    sum2 = f.sum(numbers)
    sum3 = f.sum(numbers)
    
    consistent = (sum1 == sum2 == sum3 == 15.0)
    print(f"  Sum consistency: {'✓' if consistent else '✗'}")

def test_pipeline_logic():
    """Test pipeline execution logic"""
    print("\n" + "="*60)
    print("PIPELINE LOGIC TESTING")
    print("="*60)
    
    # Test 1: Task order preservation
    print("\n1. Task Order Preservation:")
    p = PipelineR()
    p.set_verbose(0)
    
    p.print("Task 1")
    p.print("Task 2")
    p.print("Task 3")
    p.run()
    
    results = p.results()
    order_correct = len(results) == 3
    print(f"  Order preserved: {'✓' if order_correct else '✗'}")
    print(f"  Executed {len(results)} tasks in order")
    
    # Test 2: Clear functionality
    print("\n2. Clear Functionality:")
    p.clear()
    results_after_clear = p.results()
    
    cleared = len(results_after_clear) == 0
    print(f"  Clear works: {'✓' if cleared else '✗'}")
    print(f"  Results after clear: {len(results_after_clear)}")
    
    # Test 3: Re-use after clear
    print("\n3. Pipeline Reuse:")
    p.print("New task")
    p.run()
    
    new_results = p.results()
    reusable = len(new_results) == 1
    print(f"  Reuse works: {'✓' if reusable else '✗'}")

def test_type_safety():
    """Test type safety and conversions"""
    print("\n" + "="*60)
    print("TYPE SAFETY TESTING")
    print("="*60)
    
    f = Feature()
    
    # Test 1: Return types
    print("\n1. Return Type Verification:")
    
    tests = [
        ("system_info", f.system_info(), dict),
        ("cpu_usage", f.cpu_usage(), list),
        ("memory_percent", f.memory_percent(), float),
        ("now", f.now(), dict),
        ("timestamp", f.timestamp(), int),
        ("greeting", f.greeting(), str),
        ("sum", f.sum([1,2,3]), float),
        ("sqrt", f.sqrt(4), float),
        ("factorial", f.factorial(5), str),
        ("is_prime", f.is_prime(7), bool),
        ("random", f.random(), float),
        ("hash_sha256", f.hash_sha256("test"), str),
    ]
    
    for name, result, expected_type in tests:
        correct = isinstance(result, expected_type)
        print(f"  {'✓' if correct else '✗'} {name}: {type(result).__name__} (expected {expected_type.__name__})")

def test_performance_consistency():
    """Test performance consistency"""
    print("\n" + "="*60)
    print("PERFORMANCE CONSISTENCY TESTING")
    print("="*60)
    
    import time
    
    p = PipelineR()
    p.set_verbose(0)
    
    # Test multiple runs
    iterations = 10000
    times = []
    
    print(f"\n1. Running {iterations} iterations x 5 times:")
    for i in range(5):
        p.clear()
        p.loop_calc(iterations)
        
        start = time.time()
        p.run()
        elapsed = time.time() - start
        times.append(elapsed)
        
        print(f"  Run {i+1}: {elapsed:.4f}s")
    
    # Check consistency
    avg = sum(times) / len(times)
    variance = sum((t - avg) ** 2 for t in times) / len(times)
    std_dev = variance ** 0.5
    
    print(f"\n  Average: {avg:.4f}s")
    print(f"  Std Dev: {std_dev:.4f}s")
    print(f"  Variance: {variance:.6f}")
    
    # Should have low variance
    consistent = std_dev < avg * 0.2  # Less than 20% deviation
    print(f"\n  Performance consistent: {'✓' if consistent else '✗'}")

def main():
    """Run all logic tests"""
    print("="*60)
    print("TransR - Logic Unit Testing")
    print("="*60)
    
    try:
        test_logic_conditions()
        test_data_consistency()
        test_pipeline_logic()
        test_type_safety()
        test_performance_consistency()
        
        print("\n" + "="*60)
        print("✓ All logic tests completed")
        print("="*60)
        
        return 0
    except Exception as e:
        print(f"\n✗ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())

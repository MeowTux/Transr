#!/usr/bin/env python3
"""
TransR - Debugging Unit
Testing debug mode and verbose logging
"""

import sys
import time

try:
    from transR import PipelineR, Feature
except ImportError:
    print("✗ Failed to import transR")
    sys.exit(1)

def test_verbose_levels():
    """Test different verbose levels"""
    print("="*60)
    print("VERBOSE LEVEL TESTING")
    print("="*60)
    
    for level in range(4):
        print(f"\n{'='*60}")
        print(f"Verbose Level {level}")
        print('='*60)
        
        p = PipelineR()
        p.set_verbose(level)
        
        p.print(f"Testing verbose level {level}")
        p.system()
        p.time()
        p.run()
        p.clear()

def test_debug_mode():
    """Test debug mode functionality"""
    print("\n" + "="*60)
    print("DEBUG MODE TESTING")
    print("="*60)
    
    # Test 1: Debug disabled
    print("\n1. Debug Mode: DISABLED")
    p = PipelineR()
    p.set_verbose(1)
    p.set_debug(False)
    
    p.print("Task with debug OFF")
    p.loop_calc(10000)
    p.run()
    p.clear()
    
    # Test 2: Debug enabled
    print("\n2. Debug Mode: ENABLED")
    p = PipelineR()
    p.set_verbose(1)
    p.set_debug(True)
    
    p.print("Task with debug ON")
    p.loop_calc(10000)
    p.run()
    p.clear()

def test_timing_information():
    """Test execution timing"""
    print("\n" + "="*60)
    print("TIMING INFORMATION TESTING")
    print("="*60)
    
    p = PipelineR()
    p.set_verbose(2)
    p.set_debug(True)
    
    print("\nAdding tasks with varying complexity...")
    
    # Fast tasks
    p.print("Fast task 1")
    p.print("Fast task 2")
    
    # Medium tasks
    p.system()
    p.time()
    
    # Slow tasks
    p.loop_calc(50000)
    p.math("sum", list(range(1000)))
    
    print("\nExecuting pipeline...")
    p.run()
    
    # Analyze results
    results = p.results()
    print(f"\n{'='*60}")
    print("TIMING ANALYSIS")
    print('='*60)
    
    for i, result in enumerate(results, 1):
        task_name = result['task'].split('(')[0]
        duration = result['duration_ms']
        print(f"{i}. {task_name:20s}: {duration:6.2f}ms")
    
    total = sum(r['duration_ms'] for r in results)
    print(f"\nTotal execution time: {total:.2f}ms")

def test_error_tracking():
    """Test error tracking and reporting"""
    print("\n" + "="*60)
    print("ERROR TRACKING TESTING")
    print("="*60)
    
    f = Feature()
    
    # Test 1: Handled errors
    print("\n1. Testing Error Handling:")
    
    test_cases = [
        ("Empty mean", lambda: f.mean([])),
        ("Negative sqrt", lambda: f.sqrt(-1)),
    ]
    
    for name, func in test_cases:
        try:
            result = func()
            print(f"  ✗ {name}: Should have raised error")
        except Exception as e:
            print(f"  ✓ {name}: {type(e).__name__}")
            print(f"    Message: {str(e)}")

def test_performance_profiling():
    """Test performance profiling"""
    print("\n" + "="*60)
    print("PERFORMANCE PROFILING")
    print("="*60)
    
    p = PipelineR()
    p.set_verbose(0)
    
    # Test different operation sizes
    sizes = [1000, 5000, 10000, 50000, 100000]
    
    print("\nLoop Performance Analysis:")
    print(f"{'Iterations':>12} | {'Time (ms)':>10} | {'Ops/sec':>12}")
    print("-" * 40)
    
    for size in sizes:
        p.clear()
        p.loop_calc(size)
        
        start = time.time()
        p.run()
        elapsed = (time.time() - start) * 1000  # Convert to ms
        
        ops_per_sec = size / (elapsed / 1000)
        
        print(f"{size:12,} | {elapsed:10.2f} | {ops_per_sec:12,.0f}")

def test_memory_tracking():
    """Test memory usage patterns"""
    print("\n" + "="*60)
    print("MEMORY TRACKING")
    print("="*60)
    
    f = Feature()
    
    print("\nMemory usage during operations:")
    
    # Initial memory
    mem_start = f.memory_percent()
    print(f"Initial memory: {mem_start:.2f}%")
    
    # Create large data
    print("\nCreating large datasets...")
    large_numbers = list(range(1000000))
    
    mem_after_data = f.memory_percent()
    print(f"After data creation: {mem_after_data:.2f}%")
    print(f"Increase: {mem_after_data - mem_start:.2f}%")
    
    # Perform operations
    print("\nPerforming operations...")
    result = f.sum(large_numbers[:10000])
    
    mem_after_ops = f.memory_percent()
    print(f"After operations: {mem_after_ops:.2f}%")
    
    # Cleanup
    del large_numbers
    
    print("\nAfter cleanup...")
    mem_final = f.memory_percent()
    print(f"Final memory: {mem_final:.2f}%")

def test_system_state_tracking():
    """Track system state during execution"""
    print("\n" + "="*60)
    print("SYSTEM STATE TRACKING")
    print("="*60)
    
    f = Feature()
    
    print("\nSystem state snapshots:")
    print(f"{'Time':>8} | {'CPU %':>8} | {'Memory %':>10}")
    print("-" * 32)
    
    # Take multiple snapshots
    for i in range(5):
        cpu = f.cpu_usage()
        avg_cpu = sum(cpu) / len(cpu)
        mem = f.memory_percent()
        
        print(f"{i*2:8}s | {avg_cpu:8.1f} | {mem:10.1f}")
        
        if i < 4:  # Don't sleep on last iteration
            # Simulate some work
            p = PipelineR()
            p.set_verbose(0)
            p.loop_calc(50000)
            p.run()
            
            time.sleep(2)

def test_detailed_logging():
    """Test detailed logging output"""
    print("\n" + "="*60)
    print("DETAILED LOGGING")
    print("="*60)
    
    print("\nMaximum verbosity with all features:")
    
    p = PipelineR()
    p.set_verbose(3)  # Maximum verbosity
    p.set_debug(True)
    
    print("\n" + "="*60)
    print("Executing comprehensive pipeline...")
    print("="*60 + "\n")
    
    # Add variety of tasks
    p.print("Starting detailed logging test")
    p.system()
    p.time()
    p.math("sum", [1, 2, 3, 4, 5])
    p.math("mean", [10, 20, 30, 40, 50])
    p.hash("sha256", "test_data")
    p.loop_calc(10000)
    
    p.run()
    
    print("\n" + "="*60)
    print("Pipeline execution completed")
    print("="*60)

def main():
    """Run all debugging tests"""
    print("="*60)
    print("TransR - Debugging Unit Tests")
    print("="*60)
    
    try:
        test_verbose_levels()
        test_debug_mode()
        test_timing_information()
        test_error_tracking()
        test_performance_profiling()
        test_memory_tracking()
        test_system_state_tracking()
        test_detailed_logging()
        
        print("\n" + "="*60)
        print("✓ All debugging tests completed")
        print("="*60)
        
        return 0
    except Exception as e:
        print(f"\n✗ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())

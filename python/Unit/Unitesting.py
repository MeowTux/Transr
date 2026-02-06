#!/usr/bin/env python3
"""
TransR - Unit Testing
Comprehensive unit tests for all functions and plugins
"""

import unittest
import sys

try:
    from transR import PipelineR, Feature
except ImportError:
    print("✗ Failed to import transR")
    sys.exit(1)

class TestPipelineR(unittest.TestCase):
    """Test PipelineR class"""
    
    def setUp(self):
        self.pipeline = PipelineR()
    
    def test_pipeline_creation(self):
        """Test pipeline can be created"""
        self.assertIsInstance(self.pipeline, PipelineR)
    
    def test_set_verbose(self):
        """Test verbose level setting"""
        for level in range(4):
            self.pipeline.set_verbose(level)
    
    def test_set_debug(self):
        """Test debug mode"""
        self.pipeline.set_debug(True)
        self.pipeline.set_debug(False)
    
    def test_print_task(self):
        """Test print task"""
        self.pipeline.print("test")
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_system_task(self):
        """Test system check task"""
        self.pipeline.system()
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_time_task(self):
        """Test time check task"""
        self.pipeline.time()
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_math_task(self):
        """Test math task"""
        self.pipeline.math("sum", [1, 2, 3])
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_hash_task(self):
        """Test hash task"""
        self.pipeline.hash("sha256", "test")
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_loop_task(self):
        """Test loop calculation"""
        self.pipeline.loop_calc(1000)
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_blockchain_task(self):
        """Test blockchain simulation"""
        self.pipeline.blockchain(10, 10)
        self.pipeline.run()
        self.pipeline.clear()
    
    def test_multiple_tasks(self):
        """Test multiple tasks in pipeline"""
        self.pipeline.print("test1")
        self.pipeline.print("test2")
        self.pipeline.system()
        self.pipeline.time()
        self.pipeline.run()
        
        results = self.pipeline.results()
        self.assertEqual(len(results), 4)
        self.pipeline.clear()
    
    def test_results(self):
        """Test getting results"""
        self.pipeline.print("test")
        self.pipeline.run()
        
        results = self.pipeline.results()
        self.assertIsInstance(results, list)
        self.assertGreater(len(results), 0)
        self.pipeline.clear()

class TestSystemFunctions(unittest.TestCase):
    """Test system functions"""
    
    def setUp(self):
        self.feature = Feature()
    
    def test_system_info(self):
        """Test system info"""
        info = self.feature.system_info()
        
        self.assertIsInstance(info, dict)
        self.assertIn('os_name', info)
        self.assertIn('cpu_count', info)
        self.assertIn('total_memory', info)
    
    def test_cpu_usage(self):
        """Test CPU usage"""
        usage = self.feature.cpu_usage()
        
        self.assertIsInstance(usage, list)
        self.assertGreater(len(usage), 0)
        
        for u in usage:
            self.assertGreaterEqual(u, 0.0)
            self.assertLessEqual(u, 100.0)
    
    def test_memory_percent(self):
        """Test memory percentage"""
        mem = self.feature.memory_percent()
        
        self.assertIsInstance(mem, float)
        self.assertGreaterEqual(mem, 0.0)
        self.assertLessEqual(mem, 100.0)

class TestTimeFunctions(unittest.TestCase):
    """Test time functions"""
    
    def setUp(self):
        self.feature = Feature()
    
    def test_now(self):
        """Test current time"""
        now = self.feature.now()
        
        self.assertIsInstance(now, dict)
        self.assertIn('year', now)
        self.assertIn('month', now)
        self.assertIn('day', now)
        self.assertIn('hour', now)
        self.assertIn('minute', now)
        self.assertIn('second', now)
        self.assertIn('weekday', now)
        self.assertIn('formatted', now)
        
        # Validate ranges
        self.assertGreater(now['year'], 2020)
        self.assertGreaterEqual(now['month'], 1)
        self.assertLessEqual(now['month'], 12)
        self.assertGreaterEqual(now['day'], 1)
        self.assertLessEqual(now['day'], 31)
        self.assertGreaterEqual(now['hour'], 0)
        self.assertLessEqual(now['hour'], 23)
    
    def test_timestamp(self):
        """Test timestamp"""
        ts = self.feature.timestamp()
        
        self.assertIsInstance(ts, int)
        self.assertGreater(ts, 1600000000)  # After 2020
    
    def test_greeting(self):
        """Test greeting"""
        greeting = self.feature.greeting()
        
        self.assertIsInstance(greeting, str)
        self.assertIn('Good', greeting)

class TestMathFunctions(unittest.TestCase):
    """Test math functions"""
    
    def setUp(self):
        self.feature = Feature()
    
    def test_sum(self):
        """Test sum"""
        result = self.feature.sum([1, 2, 3, 4, 5])
        self.assertEqual(result, 15.0)
    
    def test_mean(self):
        """Test mean"""
        result = self.feature.mean([10, 20, 30])
        self.assertEqual(result, 20.0)
    
    def test_sqrt(self):
        """Test square root"""
        self.assertEqual(self.feature.sqrt(4), 2.0)
        self.assertEqual(self.feature.sqrt(16), 4.0)
        self.assertEqual(self.feature.sqrt(25), 5.0)
    
    def test_sqrt_negative(self):
        """Test sqrt with negative number"""
        with self.assertRaises(Exception):
            self.feature.sqrt(-1)
    
    def test_mean_empty(self):
        """Test mean with empty list"""
        with self.assertRaises(Exception):
            self.feature.mean([])
    
    def test_factorial(self):
        """Test factorial"""
        self.assertEqual(self.feature.factorial(0), "1")
        self.assertEqual(self.feature.factorial(1), "1")
        self.assertEqual(self.feature.factorial(5), "120")
    
    def test_is_prime(self):
        """Test prime checking"""
        # Prime numbers
        self.assertTrue(self.feature.is_prime(2))
        self.assertTrue(self.feature.is_prime(3))
        self.assertTrue(self.feature.is_prime(5))
        self.assertTrue(self.feature.is_prime(17))
        
        # Non-prime numbers
        self.assertFalse(self.feature.is_prime(1))
        self.assertFalse(self.feature.is_prime(4))
        self.assertFalse(self.feature.is_prime(9))
        self.assertFalse(self.feature.is_prime(15))
    
    def test_random(self):
        """Test random number generation"""
        for _ in range(10):
            r = self.feature.random()
            self.assertGreaterEqual(r, 0.0)
            self.assertLessEqual(r, 1.0)
    
    def test_random_range(self):
        """Test random range"""
        for _ in range(10):
            r = self.feature.random_range(1, 100)
            self.assertGreaterEqual(r, 1)
            self.assertLessEqual(r, 100)

class TestCryptoFunctions(unittest.TestCase):
    """Test cryptographic functions"""
    
    def setUp(self):
        self.feature = Feature()
    
    def test_sha256(self):
        """Test SHA256 hashing"""
        hash1 = self.feature.hash_sha256("test")
        hash2 = self.feature.hash_sha256("test")
        hash3 = self.feature.hash_sha256("different")
        
        # Same input = same hash
        self.assertEqual(hash1, hash2)
        
        # Different input = different hash
        self.assertNotEqual(hash1, hash3)
        
        # SHA256 is 64 hex characters
        self.assertEqual(len(hash1), 64)
    
    def test_md5(self):
        """Test MD5 hashing"""
        hash1 = self.feature.hash_md5("test")
        hash2 = self.feature.hash_md5("test")
        hash3 = self.feature.hash_md5("different")
        
        # Same input = same hash
        self.assertEqual(hash1, hash2)
        
        # Different input = different hash
        self.assertNotEqual(hash1, hash3)
        
        # MD5 is 32 hex characters
        self.assertEqual(len(hash1), 32)

class TestPortScanner(unittest.TestCase):
    """Test port scanner"""
    
    def setUp(self):
        self.feature = Feature()
    
    def test_scan_port(self):
        """Test single port scan"""
        result = self.feature.scan_port("127.0.0.1", 80)
        
        self.assertIsInstance(result, dict)
        self.assertIn('host', result)
        self.assertIn('port', result)
        self.assertIn('is_open', result)
        self.assertIn('service', result)
        
        self.assertEqual(result['host'], "127.0.0.1")
        self.assertEqual(result['port'], 80)
        self.assertIsInstance(result['is_open'], bool)

def run_tests():
    """Run all tests"""
    # Create test suite
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # Add test cases
    suite.addTests(loader.loadTestsFromTestCase(TestPipelineR))
    suite.addTests(loader.loadTestsFromTestCase(TestSystemFunctions))
    suite.addTests(loader.loadTestsFromTestCase(TestTimeFunctions))
    suite.addTests(loader.loadTestsFromTestCase(TestMathFunctions))
    suite.addTests(loader.loadTestsFromTestCase(TestCryptoFunctions))
    suite.addTests(loader.loadTestsFromTestCase(TestPortScanner))
    
    # Run tests
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    # Return exit code
    return 0 if result.wasSuccessful() else 1

if __name__ == '__main__':
    sys.exit(run_tests())

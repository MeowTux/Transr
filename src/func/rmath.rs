use crate::errhuman::{TransRError, TransRResult};
use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One, ToPrimitive};
use rand::Rng;

pub struct RMath;

impl RMath {
    // Basic operations
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
    
    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }
    
    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }
    
    pub fn div(a: f64, b: f64) -> TransRResult<f64> {
        if b == 0.0 {
            Err(TransRError::MathError("Division by zero".to_string()))
        } else {
            Ok(a / b)
        }
    }
    
    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }
    
    pub fn sqrt(x: f64) -> TransRResult<f64> {
        if x < 0.0 {
            Err(TransRError::MathError("Cannot calculate square root of negative number".to_string()))
        } else {
            Ok(x.sqrt())
        }
    }
    
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }
    
    // Trigonometric functions
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }
    
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }
    
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }
    
    // Logarithmic functions
    pub fn log(x: f64, base: f64) -> TransRResult<f64> {
        if x <= 0.0 {
            Err(TransRError::MathError("Logarithm of non-positive number".to_string()))
        } else {
            Ok(x.log(base))
        }
    }
    
    pub fn ln(x: f64) -> TransRResult<f64> {
        if x <= 0.0 {
            Err(TransRError::MathError("Natural logarithm of non-positive number".to_string()))
        } else {
            Ok(x.ln())
        }
    }
    
    // Statistical functions
    pub fn sum(numbers: &[f64]) -> f64 {
        numbers.iter().sum()
    }
    
    pub fn mean(numbers: &[f64]) -> TransRResult<f64> {
        if numbers.is_empty() {
            Err(TransRError::MathError("Cannot calculate mean of empty array".to_string()))
        } else {
            Ok(Self::sum(numbers) / numbers.len() as f64)
        }
    }
    
    pub fn median(numbers: &mut [f64]) -> TransRResult<f64> {
        if numbers.is_empty() {
            return Err(TransRError::MathError("Cannot calculate median of empty array".to_string()));
        }
        
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = numbers.len() / 2;
        
        if numbers.len() % 2 == 0 {
            Ok((numbers[mid - 1] + numbers[mid]) / 2.0)
        } else {
            Ok(numbers[mid])
        }
    }
    
    pub fn variance(numbers: &[f64]) -> TransRResult<f64> {
        if numbers.is_empty() {
            return Err(TransRError::MathError("Cannot calculate variance of empty array".to_string()));
        }
        
        let mean = Self::mean(numbers)?;
        let variance = numbers.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / numbers.len() as f64;
        
        Ok(variance)
    }
    
    pub fn std_dev(numbers: &[f64]) -> TransRResult<f64> {
        Ok(Self::variance(numbers)?.sqrt())
    }
    
    pub fn min(numbers: &[f64]) -> TransRResult<f64> {
        numbers.iter()
            .copied()
            .reduce(f64::min)
            .ok_or_else(|| TransRError::MathError("Cannot find min of empty array".to_string()))
    }
    
    pub fn max(numbers: &[f64]) -> TransRResult<f64> {
        numbers.iter()
            .copied()
            .reduce(f64::max)
            .ok_or_else(|| TransRError::MathError("Cannot find max of empty array".to_string()))
    }
    
    // Number theory
    pub fn factorial(n: u64) -> BigInt {
        if n == 0 || n == 1 {
            return BigInt::one();
        }
        
        (2..=n)
            .map(|i| i.to_bigint().unwrap())
            .product()
    }
    
    pub fn gcd(a: i64, b: i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();
        
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    
    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            0
        } else {
            (a.abs() * b.abs()) / Self::gcd(a, b)
        }
    }
    
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let limit = (n as f64).sqrt() as u64;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    
    pub fn fibonacci(n: usize) -> Vec<u64> {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }
        
        let mut fib = vec![0, 1];
        for i in 2..n {
            let next = fib[i - 1] + fib[i - 2];
            fib.push(next);
        }
        fib
    }
    
    // Random number generation
    pub fn random() -> f64 {
        rand::thread_rng().gen()
    }
    
    pub fn random_range(min: i64, max: i64) -> i64 {
        rand::thread_rng().gen_range(min..=max)
    }
    
    pub fn random_float(min: f64, max: f64) -> f64 {
        rand::thread_rng().gen_range(min..=max)
    }
    
    // Rounding functions
    pub fn round(x: f64, decimals: u32) -> f64 {
        let multiplier = 10_f64.powi(decimals as i32);
        (x * multiplier).round() / multiplier
    }
    
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }
    
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

pub struct DebugMode;

impl DebugMode {
    pub fn enable() {
        DEBUG_MODE.store(true, Ordering::Relaxed);
        println!("🐛 Debug Mode: ENABLED");
    }
    
    pub fn disable() {
        DEBUG_MODE.store(false, Ordering::Relaxed);
        println!("🐛 Debug Mode: DISABLED");
    }
    
    pub fn is_enabled() -> bool {
        DEBUG_MODE.load(Ordering::Relaxed)
    }
    
    pub fn log(msg: &str) {
        if Self::is_enabled() {
            println!("🐛 [DEBUG] {}", msg);
        }
    }
    
    pub fn log_value<T: std::fmt::Debug>(name: &str, value: &T) {
        if Self::is_enabled() {
            println!("🐛 [DEBUG] {} = {:?}", name, value);
        }
    }
    
    pub fn measure<F, R>(name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if Self::is_enabled() {
            let start = Instant::now();
            let result = f();
            let duration = start.elapsed();
            println!("⏱️  [BENCHMARK] {} took {:?}", name, duration);
            result
        } else {
            f()
        }
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::debug::DebugMode::is_enabled() {
            println!("🐛 [DEBUG] {}", format!($($arg)*));
        }
    };
}

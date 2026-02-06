use std::sync::atomic::{AtomicU8, Ordering};

static VERBOSE_LEVEL: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerboseLevel {
    Silent = 0,
    Info = 1,
    Debug = 2,
    Trace = 3,
}

impl From<u8> for VerboseLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => VerboseLevel::Silent,
            1 => VerboseLevel::Info,
            2 => VerboseLevel::Debug,
            _ => VerboseLevel::Trace,
        }
    }
}

pub struct Verbose;

impl Verbose {
    pub fn set_level(level: u8) {
        VERBOSE_LEVEL.store(level.min(3), Ordering::Relaxed);
    }
    
    pub fn get_level() -> VerboseLevel {
        VERBOSE_LEVEL.load(Ordering::Relaxed).into()
    }
    
    pub fn info(msg: &str) {
        if VERBOSE_LEVEL.load(Ordering::Relaxed) >= 1 {
            println!("ℹ️  [INFO] {}", msg);
        }
    }
    
    pub fn debug(msg: &str) {
        if VERBOSE_LEVEL.load(Ordering::Relaxed) >= 2 {
            println!("🔍 [DEBUG] {}", msg);
        }
    }
    
    pub fn trace(msg: &str) {
        if VERBOSE_LEVEL.load(Ordering::Relaxed) >= 3 {
            println!("🔬 [TRACE] {}", msg);
        }
    }
    
    pub fn error(msg: &str) {
        println!("❌ [ERROR] {}", msg);
    }
    
    pub fn success(msg: &str) {
        println!("✅ [SUCCESS] {}", msg);
    }
    
    pub fn warn(msg: &str) {
        println!("⚠️  [WARN] {}", msg);
    }
}

#[macro_export]
macro_rules! vvv_info {
    ($($arg:tt)*) => {
        $crate::vvv::Verbose::info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! vvv_debug {
    ($($arg:tt)*) => {
        $crate::vvv::Verbose::debug(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! vvv_trace {
    ($($arg:tt)*) => {
        $crate::vvv::Verbose::trace(&format!($($arg)*));
    };
}

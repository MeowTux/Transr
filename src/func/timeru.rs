use chrono::{DateTime, Local, Utc, Duration, NaiveDateTime, TimeZone};
use crate::errhuman::{TransRError, TransRResult};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInfo {
    pub local: String,
    pub utc: String,
    pub timestamp: i64,
    pub timezone: String,
}

pub struct TimerU {
    start_time: Option<Instant>,
}

impl TimerU {
    pub fn new() -> Self {
        TimerU { start_time: None }
    }
    
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    
    pub fn elapsed(&self) -> TransRResult<std::time::Duration> {
        self.start_time
            .map(|start| start.elapsed())
            .ok_or_else(|| TransRError::ValidationError("Timer not started".to_string()))
    }
    
    pub fn elapsed_ms(&self) -> TransRResult<u128> {
        Ok(self.elapsed()?.as_millis())
    }
    
    pub fn elapsed_secs(&self) -> TransRResult<f64> {
        Ok(self.elapsed()?.as_secs_f64())
    }
    
    pub fn reset(&mut self) {
        self.start_time = Some(Instant::now());
    }
}

pub fn now_local() -> DateTime<Local> {
    Local::now()
}

pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn timestamp_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn from_timestamp(ts: i64) -> TransRResult<DateTime<Utc>> {
    Utc.timestamp_opt(ts, 0)
        .single()
        .ok_or_else(|| TransRError::ParseError(format!("Invalid timestamp: {}", ts)))
}

pub fn format_datetime(dt: &DateTime<Local>, format: &str) -> String {
    dt.format(format).to_string()
}

pub fn parse_datetime(s: &str, format: &str) -> TransRResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, format)
        .map_err(|e| TransRError::ParseError(format!("Failed to parse datetime: {}", e)))
}

pub fn add_days(dt: DateTime<Local>, days: i64) -> DateTime<Local> {
    dt + Duration::days(days)
}

pub fn add_hours(dt: DateTime<Local>, hours: i64) -> DateTime<Local> {
    dt + Duration::hours(hours)
}

pub fn add_minutes(dt: DateTime<Local>, minutes: i64) -> DateTime<Local> {
    dt + Duration::minutes(minutes)
}

pub fn diff_seconds(dt1: DateTime<Local>, dt2: DateTime<Local>) -> i64 {
    (dt2 - dt1).num_seconds()
}

pub fn get_time_info() -> TimeInfo {
    let local = now_local();
    let utc = now_utc();
    
    TimeInfo {
        local: local.to_rfc3339(),
        utc: utc.to_rfc3339(),
        timestamp: timestamp(),
        timezone: local.offset().to_string(),
    }
}

pub struct Benchmark {
    name: String,
    start: Instant,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Benchmark {
            name: name.to_string(),
            start: Instant::now(),
        }
    }
    
    pub fn finish(&self) -> std::time::Duration {
        let duration = self.start.elapsed();
        println!("⏱️  [BENCHMARK] {} took {:?}", self.name, duration);
        duration
    }
}

impl Default for TimerU {
    fn default() -> Self {
        Self::new()
    }
}

use chrono::{DateTime, Local, Utc, Datelike, Timelike};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub weekday: String,
    pub iso_week: u32,
    pub formatted: String,
}

impl NowTime {
    pub fn now() -> Self {
        let dt = Local::now();
        
        NowTime {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
            weekday: dt.weekday().to_string(),
            iso_week: dt.iso_week().week(),
            formatted: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
    
    pub fn utc() -> Self {
        let dt = Utc::now();
        
        NowTime {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
            weekday: dt.weekday().to_string(),
            iso_week: dt.iso_week().week(),
            formatted: dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }
    }
    
    pub fn custom_format(&self, format: &str) -> String {
        let dt = Local::now();
        dt.format(format).to_string()
    }
    
    pub fn is_morning(&self) -> bool {
        self.hour >= 5 && self.hour < 12
    }
    
    pub fn is_afternoon(&self) -> bool {
        self.hour >= 12 && self.hour < 17
    }
    
    pub fn is_evening(&self) -> bool {
        self.hour >= 17 && self.hour < 21
    }
    
    pub fn is_night(&self) -> bool {
        self.hour >= 21 || self.hour < 5
    }
    
    pub fn is_weekend(&self) -> bool {
        self.weekday == "Sat" || self.weekday == "Sun"
    }
    
    pub fn time_of_day(&self) -> String {
        if self.is_morning() {
            "Morning".to_string()
        } else if self.is_afternoon() {
            "Afternoon".to_string()
        } else if self.is_evening() {
            "Evening".to_string()
        } else {
            "Night".to_string()
        }
    }
    
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}, {} {}:{}:{} ({})",
            self.weekday,
            self.month_name(),
            self.day,
            self.year,
            self.hour,
            self.minute,
            self.second,
            self.time_of_day()
        )
    }
    
    fn month_name(&self) -> &str {
        match self.month {
            1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
            5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
            9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
            _ => "Unknown"
        }
    }
}

pub fn greeting() -> String {
    let now = NowTime::now();
    let time = now.time_of_day();
    
    match time.as_str() {
        "Morning" => "☀️  Good Morning!".to_string(),
        "Afternoon" => "🌤️  Good Afternoon!".to_string(),
        "Evening" => "🌆 Good Evening!".to_string(),
        "Night" => "🌙 Good Night!".to_string(),
        _ => "👋 Hello!".to_string(),
    }
}

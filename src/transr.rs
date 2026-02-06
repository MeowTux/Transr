use crate::errhuman::{TransRError, TransRResult};
use crate::vvv::Verbose;
use crate::debug::DebugMode;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Task {
    Print(String),
    SystemCheck,
    TimeCheck,
    MathCalc { operation: String, args: Vec<f64> },
    NetworkRequest { method: String, url: String, data: Option<String> },
    PortScan { host: String, ports: Vec<u16> },
    VulnScan { url: String },
    CryptoHash { algorithm: String, data: String },
    Loop { iterations: u32, operation: String },
    Custom { name: String, args: HashMap<String, String> },
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub task_name: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u128,
}

pub struct TransRPipeline {
    pub tasks: Vec<Task>,
    pub results: Vec<PipelineResult>,
    pub verbose_level: u8,
    pub debug_mode: bool,
}

impl TransRPipeline {
    pub fn new() -> Self {
        TransRPipeline {
            tasks: Vec::new(),
            results: Vec::new(),
            verbose_level: 1,
            debug_mode: false,
        }
    }
    
    pub fn set_verbose(&mut self, level: u8) {
        self.verbose_level = level;
        Verbose::set_level(level);
    }
    
    pub fn set_debug(&mut self, enabled: bool) {
        self.debug_mode = enabled;
        if enabled {
            DebugMode::enable();
        } else {
            DebugMode::disable();
        }
    }
    
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
    pub fn print(&mut self, text: String) {
        self.add_task(Task::Print(text));
    }
    
    pub fn system_check(&mut self) {
        self.add_task(Task::SystemCheck);
    }
    
    pub fn time_check(&mut self) {
        self.add_task(Task::TimeCheck);
    }
    
    pub fn math(&mut self, operation: String, args: Vec<f64>) {
        self.add_task(Task::MathCalc { operation, args });
    }
    
    pub fn http_get(&mut self, url: String) {
        self.add_task(Task::NetworkRequest {
            method: "GET".to_string(),
            url,
            data: None,
        });
    }
    
    pub fn port_scan(&mut self, host: String, ports: Vec<u16>) {
        self.add_task(Task::PortScan { host, ports });
    }
    
    pub fn vuln_scan(&mut self, url: String) {
        self.add_task(Task::VulnScan { url });
    }
    
    pub fn hash(&mut self, algorithm: String, data: String) {
        self.add_task(Task::CryptoHash { algorithm, data });
    }
    
    pub fn heavy_loop(&mut self, iterations: u32) {
        self.add_task(Task::Loop {
            iterations,
            operation: "xor".to_string(),
        });
    }
    
    pub fn blockchain_loop(&mut self, height: u32, weight: u32) {
        let total_iterations = height * weight;
        self.add_task(Task::Loop {
            iterations: total_iterations,
            operation: "blockchain".to_string(),
        });
    }
    
    pub fn clear(&mut self) {
        self.tasks.clear();
        self.results.clear();
    }
    
    pub fn run(&mut self) -> TransRResult<()> {
        Verbose::info("🚀 [TransR Engine]: Pipeline Starting...");
        
        let total_tasks = self.tasks.len();
        
        for (idx, task) in self.tasks.iter().enumerate() {
            Verbose::debug(&format!("Task {}/{}: {:?}", idx + 1, total_tasks, task));
            
            let start = std::time::Instant::now();
            let result = self.execute_task(task)?;
            let duration = start.elapsed().as_millis();
            
            self.results.push(PipelineResult {
                task_name: format!("{:?}", task),
                success: true,
                output: result,
                duration_ms: duration,
            });
        }
        
        self.tasks.clear();
        Verbose::success(&format!("✅ [TransR Engine]: Pipeline Finished. Executed {} tasks.", total_tasks));
        
        Ok(())
    }
    
    fn execute_task(&self, task: &Task) -> TransRResult<String> {
        match task {
            Task::Print(text) => {
                println!("📝 Output: {}", text);
                Ok(format!("Printed: {}", text))
            }
            
            Task::SystemCheck => {
                use crate::func::sysez::SysEz;
                let mut sys = SysEz::new();
                let info = sys.get_system_info()?;
                
                println!("⚙️  System: {} ({})", info.os_name, info.os_version);
                println!("   CPU: {} ({}cores)", info.cpu_brand, info.cpu_count);
                println!("   RAM: {:.2}GB / {:.2}GB", 
                    info.used_memory as f64 / 1024.0 / 1024.0 / 1024.0,
                    info.total_memory as f64 / 1024.0 / 1024.0 / 1024.0
                );
                
                Ok("System check completed".to_string())
            }
            
            Task::TimeCheck => {
                use crate::func::nowtime::NowTime;
                let now = NowTime::now();
                
                println!("🕐 Time: {}", now.formatted);
                println!("   Period: {}", now.time_of_day());
                println!("   Week: {}", now.iso_week);
                
                Ok(format!("Time: {}", now.formatted))
            }
            
            Task::MathCalc { operation, args } => {
                use crate::func::rmath::RMath;
                
                let result = match operation.as_str() {
                    "sum" => RMath::sum(args),
                    "mean" => RMath::mean(args)?,
                    "max" => RMath::max(args)?,
                    "min" => RMath::min(args)?,
                    _ => return Err(TransRError::MathError(format!("Unknown operation: {}", operation))),
                };
                
                println!("🔢 Math Result ({}): {}", operation, result);
                Ok(format!("{}", result))
            }
            
            Task::NetworkRequest { method, url, data } => {
                use crate::func::rnet::RNet;
                let client = RNet::new()?;
                
                let response = match method.as_str() {
                    "GET" => client.get(url)?,
                    "POST" => client.post(url, data.as_ref().unwrap_or(&"{}".to_string()))?,
                    _ => return Err(TransRError::NetworkError(format!("Unsupported method: {}", method))),
                };
                
                println!("🌐 HTTP Response: Status {}", response.status);
                Ok(format!("Status: {}", response.status))
            }
            
            Task::PortScan { host, ports } => {
                use crate::plugins::rustnetx::RustNetX;
                let scanner = RustNetX::new();
                let results = scanner.scan_ports(host, ports.clone());
                
                let open_count = results.iter().filter(|r| r.is_open).count();
                println!("🔍 Port Scan: {} open / {} total", open_count, results.len());
                
                Ok(format!("{} open ports", open_count))
            }
            
            Task::VulnScan { url } => {
                use crate::plugins::hyros::HyrOS;
                let scanner = HyrOS::new()?;
                let results = scanner.scan(url);
                
                let vuln_count = results.iter().filter(|r| r.matched).count();
                println!("🔐 Vulnerability Scan: {} vulnerabilities found", vuln_count);
                
                Ok(format!("{} vulnerabilities", vuln_count))
            }
            
            Task::CryptoHash { algorithm, data } => {
                use sha2::{Sha256, Digest};
                use md5::Md5;
                
                let hash = match algorithm.as_str() {
                    "sha256" => {
                        let mut hasher = Sha256::new();
                        hasher.update(data.as_bytes());
                        format!("{:x}", hasher.finalize())
                    }
                    "md5" => {
                        let mut hasher = Md5::new();
                        hasher.update(data.as_bytes());
                        format!("{:x}", hasher.finalize())
                    }
                    _ => return Err(TransRError::CryptoError(format!("Unknown algorithm: {}", algorithm))),
                };
                
                println!("🔐 Hash ({}): {}", algorithm, hash);
                Ok(hash)
            }
            
            Task::Loop { iterations, operation } => {
                let mut val: u32 = 0;
                
                match operation.as_str() {
                    "xor" => {
                        for i in 0..*iterations {
                            val = (val.wrapping_add(i)) ^ 0x12345678;
                        }
                    }
                    "blockchain" => {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        
                        for _ in 0..*iterations {
                            val = (val.wrapping_add(rng.gen::<u32>())) ^ 0xDEADBEEF;
                        }
                    }
                    _ => {
                        for i in 0..*iterations {
                            val = val.wrapping_add(i);
                        }
                    }
                }
                
                println!("🔢 Loop Result ({} iterations): {}", iterations, val);
                Ok(format!("{}", val))
            }
            
            Task::Custom { name, args } => {
                println!("⚡ Custom Task: {}", name);
                println!("   Args: {:?}", args);
                Ok(format!("Executed: {}", name))
            }
        }
    }
    
    pub fn get_results(&self) -> &[PipelineResult] {
        &self.results
    }
}

impl Default for TransRPipeline {
    fn default() -> Self {
        Self::new()
    }
}

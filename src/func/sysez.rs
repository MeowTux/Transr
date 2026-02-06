use sysinfo::{System, Disks, Cpu, Disk, Networks, Process, Pid}; 
use crate::errhuman::{TransRResult};
use crate::vvv::Verbose;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub cpu_count: usize,
    pub cpu_brand: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub file_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

pub struct SysEz {
    system: System,
}

impl SysEz {
    pub fn new() -> Self {
        Verbose::debug("Initializing SysEz...");
        let mut sys = System::new_all();
        sys.refresh_all();
        SysEz { system: sys }
    }
    
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }
    
    pub fn get_system_info(&mut self) -> TransRResult<SystemInfo> {
        self.refresh();
        
        let cpu_brand = self.system.cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(SystemInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            cpu_count: self.system.cpus().len(),
            cpu_brand,
            total_memory: self.system.total_memory(),
            used_memory: self.system.used_memory(),
            total_swap: self.system.total_swap(),
            used_swap: self.system.used_swap(),
            uptime: System::uptime(),
        })
    }
    
    pub fn get_disks(&mut self) -> Vec<DiskInfo> {
        let disks = Disks::new_with_refreshed_list();  // if error use this -> self.refresh();
        
        disks.iter().map(|disk| {
            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                file_system: disk.file_system().to_string_lossy().to_string(),
            }
        }).collect()
    }
    
    pub fn get_top_processes(&mut self, limit: usize) -> Vec<ProcessInfo> {
        self.refresh();
        
        let mut processes: Vec<ProcessInfo> = self.system.processes()
            .iter()
            .map(|(pid, process)| {
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory: process.memory(),
                    status: format!("{:?}", process.status()),
                }
            })
            .collect();
        
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(limit);
        processes
    }
    
    pub fn get_cpu_usage(&mut self) -> Vec<f32> {
        self.refresh();
        self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }
    
    pub fn get_memory_percentage(&mut self) -> f64 {
        self.refresh();
        let total = self.system.total_memory() as f64;
        let used = self.system.used_memory() as f64;
        if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for SysEz {
    fn default() -> Self {
        Self::new()
    }
}

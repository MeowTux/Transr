use crate::errhuman::{TransRError, TransRResult};
use crate::vvv::Verbose;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub host: String,
    pub port: u16,
    pub is_open: bool,
    pub service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostScanResult {
    pub host: String,
    pub is_alive: bool,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
}

pub struct RustNetX {
    timeout: Duration,
}

impl RustNetX {
    pub fn new() -> Self {
        RustNetX {
            timeout: Duration::from_millis(500),
        }
    }
    
    pub fn with_timeout(timeout_ms: u64) -> Self {
        RustNetX {
            timeout: Duration::from_millis(timeout_ms),
        }
    }
    
    pub fn scan_port(&self, host: &str, port: u16) -> PortScanResult {
        let address = format!("{}:{}", host, port);
        
        Verbose::trace(&format!("Scanning {}:{}", host, port));
        
        let is_open = match TcpStream::connect_timeout(
            &address.to_socket_addrs()
                .ok()
                .and_then(|mut addrs| addrs.next())
                .unwrap_or_else(|| {
                    format!("127.0.0.1:{}", port).parse().unwrap()
                }),
            self.timeout,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };
        
        let service = if is_open {
            Some(self.identify_service(port))
        } else {
            None
        };
        
        PortScanResult {
            host: host.to_string(),
            port,
            is_open,
            service,
        }
    }
    
    pub fn scan_ports(&self, host: &str, ports: Vec<u16>) -> Vec<PortScanResult> {
        Verbose::info(&format!("🔍 Scanning {} ports on {}", ports.len(), host));
        
        ports.par_iter()
            .map(|&port| self.scan_port(host, port))
            .collect()
    }
    
    pub fn scan_port_range(&self, host: &str, start: u16, end: u16) -> Vec<PortScanResult> {
        let ports: Vec<u16> = (start..=end).collect();
        self.scan_ports(host, ports)
    }
    
    pub fn quick_scan(&self, host: &str) -> HostScanResult {
        Verbose::info(&format!("⚡ Quick scan on {}", host));
        
        // Common ports
        let common_ports = vec![
            21, 22, 23, 25, 53, 80, 110, 143, 443, 445, 
            3306, 3389, 5432, 5900, 8080, 8443
        ];
        
        let results = self.scan_ports(host, common_ports);
        
        let open_ports: Vec<u16> = results.iter()
            .filter(|r| r.is_open)
            .map(|r| r.port)
            .collect();
        
        let services: Vec<String> = results.iter()
            .filter(|r| r.is_open && r.service.is_some())
            .map(|r| format!("{}: {}", r.port, r.service.as_ref().unwrap()))
            .collect();
        
        let is_alive = !open_ports.is_empty();
        
        HostScanResult {
            host: host.to_string(),
            is_alive,
            open_ports,
            services,
        }
    }
    
    pub fn full_scan(&self, host: &str) -> HostScanResult {
        Verbose::info(&format!("🔍 Full scan on {} (this may take a while...)", host));
        
        let results = self.scan_port_range(host, 1, 1024);
        
        let open_ports: Vec<u16> = results.iter()
            .filter(|r| r.is_open)
            .map(|r| r.port)
            .collect();
        
        let services: Vec<String> = results.iter()
            .filter(|r| r.is_open && r.service.is_some())
            .map(|r| format!("{}: {}", r.port, r.service.as_ref().unwrap()))
            .collect();
        
        let is_alive = !open_ports.is_empty();
        
        HostScanResult {
            host: host.to_string(),
            is_alive,
            open_ports,
            services,
        }
    }
    
    fn identify_service(&self, port: u16) -> String {
        match port {
            20 => "FTP-DATA".to_string(),
            21 => "FTP".to_string(),
            22 => "SSH".to_string(),
            23 => "Telnet".to_string(),
            25 => "SMTP".to_string(),
            53 => "DNS".to_string(),
            80 => "HTTP".to_string(),
            110 => "POP3".to_string(),
            143 => "IMAP".to_string(),
            443 => "HTTPS".to_string(),
            445 => "SMB".to_string(),
            3306 => "MySQL".to_string(),
            3389 => "RDP".to_string(),
            5432 => "PostgreSQL".to_string(),
            5900 => "VNC".to_string(),
            6379 => "Redis".to_string(),
            8080 => "HTTP-Proxy".to_string(),
            8443 => "HTTPS-Alt".to_string(),
            9200 => "Elasticsearch".to_string(),
            27017 => "MongoDB".to_string(),
            _ => "Unknown".to_string(),
        }
    }
    
    pub fn banner_grab(&self, host: &str, port: u16) -> TransRResult<String> {
        let address = format!("{}:{}", host, port);
        
        match TcpStream::connect_timeout(
            &address.to_socket_addrs()
                .map_err(|e| TransRError::NetworkError(format!("Invalid address: {}", e)))?
                .next()
                .ok_or_else(|| TransRError::NetworkError("No address found".to_string()))?,
            self.timeout,
        ) {
            Ok(stream) => {
                use std::io::Read;
                let mut buffer = [0u8; 1024];
                
                stream.set_read_timeout(Some(self.timeout)).ok();
                
                match (&stream).read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let banner = String::from_utf8_lossy(&buffer[..n]);
                        Ok(banner.to_string())
                    }
                    _ => Ok("No banner received".to_string()),
                }
            }
            Err(e) => Err(TransRError::NetworkError(format!("Connection failed: {}", e))),
        }
    }
}

impl Default for RustNetX {
    fn default() -> Self {
        Self::new()
    }
}

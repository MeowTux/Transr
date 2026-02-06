use crate::errhuman::{TransRError, TransRResult};
use crate::vvv::Verbose;
use crate::func::rnet::RNet;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnTemplate {
    pub id: String,
    pub name: String,
    pub severity: Severity,
    pub description: String,
    pub matcher: MatcherType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn icon(&self) -> &str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🔵",
            Severity::Info => "⚪",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatcherType {
    StatusCode(u16),
    Regex(String),
    Contains(String),
    Header(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnResult {
    pub template_id: String,
    pub template_name: String,
    pub severity: Severity,
    pub url: String,
    pub matched: bool,
    pub evidence: Option<String>,
    pub timestamp: String,
}

pub struct HyrOS {
    templates: Vec<VulnTemplate>,
    client: RNet,
}

impl HyrOS {
    pub fn new() -> TransRResult<Self> {
        let mut hyros = HyrOS {
            templates: Vec::new(),
            client: RNet::new()?,
        };
        
        hyros.load_default_templates();
        Ok(hyros)
    }
    
    fn load_default_templates(&mut self) {
        // Directory Traversal
        self.templates.push(VulnTemplate {
            id: "dir-traversal-1".to_string(),
            name: "Directory Traversal - Linux".to_string(),
            severity: Severity::High,
            description: "Tests for directory traversal vulnerability".to_string(),
            matcher: MatcherType::Contains("root:x:0:0".to_string()),
        });
        
        // SQL Injection
        self.templates.push(VulnTemplate {
            id: "sql-injection-1".to_string(),
            name: "SQL Injection - Error Based".to_string(),
            severity: Severity::Critical,
            description: "Tests for SQL injection vulnerability".to_string(),
            matcher: MatcherType::Regex(r"(SQL syntax|mysql_fetch|PostgreSQL query)".to_string()),
        });
        
        // XSS
        self.templates.push(VulnTemplate {
            id: "xss-reflected-1".to_string(),
            name: "Reflected XSS".to_string(),
            severity: Severity::High,
            description: "Tests for reflected XSS vulnerability".to_string(),
            matcher: MatcherType::Contains("<script>alert".to_string()),
        });
        
        // Open Redirect
        self.templates.push(VulnTemplate {
            id: "open-redirect-1".to_string(),
            name: "Open Redirect".to_string(),
            severity: Severity::Medium,
            description: "Tests for open redirect vulnerability".to_string(),
            matcher: MatcherType::StatusCode(302),
        });
        
        // Exposed Files
        self.templates.push(VulnTemplate {
            id: "exposed-git-1".to_string(),
            name: "Exposed .git Directory".to_string(),
            severity: Severity::High,
            description: "Checks for exposed .git directory".to_string(),
            matcher: MatcherType::StatusCode(200),
        });
        
        self.templates.push(VulnTemplate {
            id: "exposed-env-1".to_string(),
            name: "Exposed .env File".to_string(),
            severity: Severity::Critical,
            description: "Checks for exposed .env file".to_string(),
            matcher: MatcherType::Contains("DB_PASSWORD".to_string()),
        });
        
        // Debug Mode
        self.templates.push(VulnTemplate {
            id: "debug-mode-1".to_string(),
            name: "Debug Mode Enabled".to_string(),
            severity: Severity::Medium,
            description: "Checks if debug mode is enabled".to_string(),
            matcher: MatcherType::Regex(r"(DEBUG|debug_mode|DEBUG_MODE).*true".to_string()),
        });
        
        // Default Credentials
        self.templates.push(VulnTemplate {
            id: "default-creds-1".to_string(),
            name: "Default Admin Credentials".to_string(),
            severity: Severity::Critical,
            description: "Tests for default admin credentials".to_string(),
            matcher: MatcherType::StatusCode(200),
        });
        
        // API Exposure
        self.templates.push(VulnTemplate {
            id: "api-exposure-1".to_string(),
            name: "Exposed API Documentation".to_string(),
            severity: Severity::Info,
            description: "Checks for exposed API documentation".to_string(),
            matcher: MatcherType::Contains("swagger".to_string()),
        });
        
        // Security Headers
        self.templates.push(VulnTemplate {
            id: "missing-sec-headers-1".to_string(),
            name: "Missing Security Headers".to_string(),
            severity: Severity::Low,
            description: "Checks for missing security headers".to_string(),
            matcher: MatcherType::Header("X-Content-Type-Options".to_string(), "".to_string()),
        });
        
        Verbose::info(&format!("✓ Loaded {} vulnerability templates", self.templates.len()));
    }
    
    pub fn scan(&self, url: &str) -> Vec<VulnResult> {
        Verbose::info(&format!("🔍 Starting vulnerability scan on: {}", url));
        
        let mut results = Vec::new();
        
        for template in &self.templates {
            Verbose::debug(&format!("Testing: {}", template.name));
            
            if let Ok(result) = self.test_template(url, template) {
                if result.matched {
                    Verbose::warn(&format!("{} VULNERABILITY FOUND: {}", 
                        template.severity.icon(), 
                        template.name
                    ));
                }
                results.push(result);
            }
        }
        
        results
    }
    
    fn test_template(&self, base_url: &str, template: &VulnTemplate) -> TransRResult<VulnResult> {
        let test_url = self.build_test_url(base_url, &template.id);
        
        let response = match self.client.get(&test_url) {
            Ok(resp) => resp,
            Err(_) => {
                return Ok(VulnResult {
                    template_id: template.id.clone(),
                    template_name: template.name.clone(),
                    severity: template.severity.clone(),
                    url: test_url,
                    matched: false,
                    evidence: None,
                    timestamp: chrono::Local::now().to_rfc3339(),
                });
            }
        };
        
        let matched = self.check_matcher(&template.matcher, &response);
        let evidence = if matched {
            Some(self.extract_evidence(&response))
        } else {
            None
        };
        
        Ok(VulnResult {
            template_id: template.id.clone(),
            template_name: template.name.clone(),
            severity: template.severity.clone(),
            url: test_url,
            matched,
            evidence,
            timestamp: chrono::Local::now().to_rfc3339(),
        })
    }
    
    fn build_test_url(&self, base_url: &str, template_id: &str) -> String {
        match template_id {
            "dir-traversal-1" => format!("{}/../../../etc/passwd", base_url),
            "sql-injection-1" => format!("{}?id=1'", base_url),
            "xss-reflected-1" => format!("{}?q=<script>alert(1)</script>", base_url),
            "exposed-git-1" => format!("{}/.git/config", base_url),
            "exposed-env-1" => format!("{}/.env", base_url),
            "api-exposure-1" => format!("{}/swagger-ui.html", base_url),
            _ => base_url.to_string(),
        }
    }
    
    fn check_matcher(&self, matcher: &MatcherType, response: &crate::func::rnet::Response) -> bool {
        match matcher {
            MatcherType::StatusCode(code) => response.status == *code,
            MatcherType::Contains(text) => response.body.contains(text),
            MatcherType::Regex(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(&response.body)
                } else {
                    false
                }
            }
            MatcherType::Header(key, value) => {
                if value.is_empty() {
                    !response.headers.contains_key(key)
                } else {
                    response.headers.get(key).map_or(false, |v| v == value)
                }
            }
        }
    }
    
    fn extract_evidence(&self, response: &crate::func::rnet::Response) -> String {
        let body_preview = if response.body.len() > 200 {
            format!("{}...", &response.body[..200])
        } else {
            response.body.clone()
        };
        
        format!("Status: {}, Body: {}", response.status, body_preview)
    }
    
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.iter()
            .map(|t| format!("{} [{}] {}", t.severity.icon(), t.id, t.name))
            .collect()
    }
    
    pub fn scan_with_severity(&self, url: &str, min_severity: Severity) -> Vec<VulnResult> {
        let all_results = self.scan(url);
        
        all_results.into_iter()
            .filter(|r| self.is_severity_sufficient(&r.severity, &min_severity))
            .collect()
    }
    
    fn is_severity_sufficient(&self, result_sev: &Severity, min_sev: &Severity) -> bool {
        let severity_order = [
            Severity::Info,
            Severity::Low,
            Severity::Medium,
            Severity::High,
            Severity::Critical,
        ];
        
        let result_index = severity_order.iter().position(|s| s == result_sev).unwrap_or(0);
        let min_index = severity_order.iter().position(|s| s == min_sev).unwrap_or(0);
        
        result_index >= min_index
    }
}

impl Default for HyrOS {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

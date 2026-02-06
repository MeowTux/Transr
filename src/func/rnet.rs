use crate::errhuman::{TransRError, TransRResult};
use crate::vvv::Verbose;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub url: String,
}

pub struct RNet {
    client: Client,
}

impl RNet {
    pub fn new() -> TransRResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("TransR/1.0")
            .build()
            .map_err(|e| TransRError::NetworkError(format!("Failed to create client: {}", e)))?;
        
        Ok(RNet { client })
    }
    
    pub fn get(&self, url: &str) -> TransRResult<Response> {
        Verbose::info(&format!("GET request to: {}", url));
        
        let response = self.client.get(url)
            .send()
            .map_err(|e| TransRError::NetworkError(format!("GET request failed: {}", e)))?;
        
        let status = response.status().as_u16();
        let url = response.url().to_string();
        
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }
        
        let body = response.text()
            .map_err(|e| TransRError::NetworkError(format!("Failed to read response body: {}", e)))?;
        
        Verbose::debug(&format!("Response status: {}", status));
        
        Ok(Response {
            status,
            body,
            headers,
            url,
        })
    }
    
    pub fn post(&self, url: &str, data: &str) -> TransRResult<Response> {
        Verbose::info(&format!("POST request to: {}", url));
        
        let response = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(data.to_string())
            .send()
            .map_err(|e| TransRError::NetworkError(format!("POST request failed: {}", e)))?;
        
        let status = response.status().as_u16();
        let url = response.url().to_string();
        
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }
        
        let body = response.text()
            .map_err(|e| TransRError::NetworkError(format!("Failed to read response body: {}", e)))?;
        
        Ok(Response {
            status,
            body,
            headers,
            url,
        })
    }
    
    pub fn put(&self, url: &str, data: &str) -> TransRResult<Response> {
        Verbose::info(&format!("PUT request to: {}", url));
        
        let response = self.client.put(url)
            .header("Content-Type", "application/json")
            .body(data.to_string())
            .send()
            .map_err(|e| TransRError::NetworkError(format!("PUT request failed: {}", e)))?;
        
        let status = response.status().as_u16();
        let url = response.url().to_string();
        
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }
        
        let body = response.text()
            .map_err(|e| TransRError::NetworkError(format!("Failed to read response body: {}", e)))?;
        
        Ok(Response {
            status,
            body,
            headers,
            url,
        })
    }
    
    pub fn delete(&self, url: &str) -> TransRResult<Response> {
        Verbose::info(&format!("DELETE request to: {}", url));
        
        let response = self.client.delete(url)
            .send()
            .map_err(|e| TransRError::NetworkError(format!("DELETE request failed: {}", e)))?;
        
        let status = response.status().as_u16();
        let url = response.url().to_string();
        
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }
        
        let body = response.text()
            .map_err(|e| TransRError::NetworkError(format!("Failed to read response body: {}", e)))?;
        
        Ok(Response {
            status,
            body,
            headers,
            url,
        })
    }
    
    pub fn download(&self, url: &str, path: &str) -> TransRResult<()> {
        Verbose::info(&format!("Downloading from {} to {}", url, path));
        
        let response = self.client.get(url)
            .send()
            .map_err(|e| TransRError::NetworkError(format!("Download failed: {}", e)))?;
        
        let content = response.bytes()
            .map_err(|e| TransRError::NetworkError(format!("Failed to read content: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| TransRError::IoError(e))?;
        
        Verbose::success(&format!("Downloaded to {}", path));
        Ok(())
    }
    
    pub fn ping(&self, url: &str) -> TransRResult<bool> {
        match self.client.head(url).timeout(Duration::from_secs(5)).send() {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

impl Default for RNet {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

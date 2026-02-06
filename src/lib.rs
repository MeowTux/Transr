mod errhuman;
mod vvv;
mod debug;
mod transr;

mod func {
    pub mod sysez;
    pub mod timeru;
    pub mod nowtime;
    pub mod rmath;
    pub mod rnet;
}

mod plugins {
    pub mod decorator;
    pub mod rustnetx;
    pub mod hyros;
}

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

// ==================== PipelineR Class ====================
#[pyclass]
struct PipelineR {
    pipeline: transr::TransRPipeline,
}

#[pymethods]
impl PipelineR {
    #[new]
    fn new() -> Self {
        PipelineR {
            pipeline: transr::TransRPipeline::new(),
        }
    }
    
    fn set_verbose(&mut self, level: u8) {
        self.pipeline.set_verbose(level);
    }
    
    fn set_debug(&mut self, enabled: bool) {
        self.pipeline.set_debug(enabled);
    }
    
    fn print(&mut self, text: String) {
        self.pipeline.print(text);
    }
    
    fn system(&mut self) {
        self.pipeline.system_check();
    }
    
    fn time(&mut self) {
        self.pipeline.time_check();
    }
    
    fn math(&mut self, operation: String, args: Vec<f64>) {
        self.pipeline.math(operation, args);
    }
    
    fn http_get(&mut self, url: String) {
        self.pipeline.http_get(url);
    }
    
    fn hash(&mut self, algorithm: String, data: String) {
        self.pipeline.hash(algorithm, data);
    }
    
    fn loop_calc(&mut self, iterations: u32) {
        self.pipeline.heavy_loop(iterations);
    }
    
    fn blockchain(&mut self, height: u32, weight: u32) {
        self.pipeline.blockchain_loop(height, weight);
    }
    
    fn scan_ports(&mut self, host: String, ports: Vec<u16>) {
        self.pipeline.port_scan(host, ports);
    }
    
    fn scan_vulns(&mut self, url: String) {
        self.pipeline.vuln_scan(url);
    }
    
    fn run(&mut self) -> PyResult<()> {
        self.pipeline.run()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
    
    fn clear(&mut self) {
        self.pipeline.clear();
    }
    
    fn results(&self, py: Python) -> PyResult<PyObject> {
        let results = self.pipeline.get_results();
        let py_list = PyList::empty(py);
        
        for result in results {
            let dict = PyDict::new(py);
            dict.set_item("task", &result.task_name)?;
            dict.set_item("success", result.success)?;
            dict.set_item("output", &result.output)?;
            dict.set_item("duration_ms", result.duration_ms)?;
            py_list.append(dict)?;
        }
        
        Ok(py_list.to_object(py))
    }
}

// ==================== Feature Module ====================
#[pyclass]
struct Feature;

#[pymethods]
impl Feature {
    #[new]
    fn new() -> Self {
        Feature
    }
    
    // System features
    #[pyo3(name = "system_info")]
    fn system_info(&self, py: Python) -> PyResult<PyObject> {
        use func::sysez::SysEz;
        
        let mut sys = SysEz::new();
        let info = sys.get_system_info()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        let dict = PyDict::new(py);
        dict.set_item("os_name", info.os_name)?;
        dict.set_item("os_version", info.os_version)?;
        dict.set_item("kernel", info.kernel_version)?;
        dict.set_item("hostname", info.hostname)?;
        dict.set_item("cpu_count", info.cpu_count)?;
        dict.set_item("cpu_brand", info.cpu_brand)?;
        dict.set_item("total_memory", info.total_memory)?;
        dict.set_item("used_memory", info.used_memory)?;
        dict.set_item("uptime", info.uptime)?;
        
        Ok(dict.to_object(py))
    }
    
    #[pyo3(name = "cpu_usage")]
    fn cpu_usage(&self) -> PyResult<Vec<f32>> {
        use func::sysez::SysEz;
        let mut sys = SysEz::new();
        Ok(sys.get_cpu_usage())
    }
    
    #[pyo3(name = "memory_percent")]
    fn memory_percent(&self) -> PyResult<f64> {
        use func::sysez::SysEz;
        let mut sys = SysEz::new();
        Ok(sys.get_memory_percentage())
    }
    
    // Time features
    #[pyo3(name = "now")]
    fn now(&self, py: Python) -> PyResult<PyObject> {
        use func::nowtime::NowTime;
        let now = NowTime::now();
        
        let dict = PyDict::new(py);
        dict.set_item("year", now.year)?;
        dict.set_item("month", now.month)?;
        dict.set_item("day", now.day)?;
        dict.set_item("hour", now.hour)?;
        dict.set_item("minute", now.minute)?;
        dict.set_item("second", now.second)?;
        dict.set_item("weekday", now.weekday.clone())?;
        dict.set_item("formatted", now.formatted.clone())?;
        dict.set_item("time_of_day", now.time_of_day().clone())?;
        
        Ok(dict.to_object(py))
    }
    
    #[pyo3(name = "timestamp")]
    fn timestamp(&self) -> i64 {
        use func::timeru::timestamp;
        timestamp()
    }
    
    #[pyo3(name = "greeting")]
    fn greeting(&self) -> String {
        use func::nowtime::greeting;
        greeting()
    }
    
    // Math features
    #[pyo3(name = "sum")]
    fn sum(&self, numbers: Vec<f64>) -> f64 {
        use func::rmath::RMath;
        RMath::sum(&numbers)
    }
    
    #[pyo3(name = "mean")]
    fn mean(&self, numbers: Vec<f64>) -> PyResult<f64> {
        use func::rmath::RMath;
        RMath::mean(&numbers)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    
    #[pyo3(name = "sqrt")]
    fn sqrt(&self, x: f64) -> PyResult<f64> {
        use func::rmath::RMath;
        RMath::sqrt(x)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    
    #[pyo3(name = "factorial")]
    fn factorial(&self, n: u64) -> String {
        use func::rmath::RMath;
        RMath::factorial(n).to_string()
    }
    
    #[pyo3(name = "is_prime")]
    fn is_prime(&self, n: u64) -> bool {
        use func::rmath::RMath;
        RMath::is_prime(n)
    }
    
    #[pyo3(name = "random")]
    fn random(&self) -> f64 {
        use func::rmath::RMath;
        RMath::random()
    }
    
    #[pyo3(name = "random_range")]
    fn random_range(&self, min: i64, max: i64) -> i64 {
        use func::rmath::RMath;
        RMath::random_range(min, max)
    }
    
    // Network features
    #[pyo3(name = "http_get")]
    fn http_get(&self, py: Python, url: String) -> PyResult<PyObject> {
        use func::rnet::RNet;
        
        let client = RNet::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        let response = client.get(&url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        let dict = PyDict::new(py);
        dict.set_item("status", response.status)?;
        dict.set_item("body", response.body)?;
        dict.set_item("url", response.url)?;
        
        Ok(dict.to_object(py))
    }
    
    #[pyo3(name = "hash_sha256")]
    fn hash_sha256(&self, data: String) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    #[pyo3(name = "hash_md5")]
    fn hash_md5(&self, data: String) -> String {
        use md5::{Md5, Digest};
        let mut hasher = Md5::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    // Scanner features
    #[pyo3(name = "scan_port")]
    fn scan_port(&self, py: Python, host: String, port: u16) -> PyResult<PyObject> {
        use plugins::rustnetx::RustNetX;
        
        let scanner = RustNetX::new();
        let result = scanner.scan_port(&host, port);
        
        let dict = PyDict::new(py);
        dict.set_item("host", result.host)?;
        dict.set_item("port", result.port)?;
        dict.set_item("is_open", result.is_open)?;
        dict.set_item("service", result.service)?;
        
        Ok(dict.to_object(py))
    }
    
    #[pyo3(name = "quick_scan")]
    fn quick_scan(&self, py: Python, host: String) -> PyResult<PyObject> {
        use plugins::rustnetx::RustNetX;
        
        let scanner = RustNetX::new();
        let result = scanner.quick_scan(&host);
        
        let dict = PyDict::new(py);
        dict.set_item("host", result.host)?;
        dict.set_item("is_alive", result.is_alive)?;
        dict.set_item("open_ports", result.open_ports)?;
        dict.set_item("services", result.services)?;
        
        Ok(dict.to_object(py))
    }
    
    #[pyo3(name = "vuln_scan")]
    fn vuln_scan(&self, py: Python, url: String) -> PyResult<PyObject> {
        use plugins::hyros::HyrOS;
        
        let scanner = HyrOS::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        let results = scanner.scan(&url);
        let py_list = PyList::empty(py);
        
        for result in results {
            if result.matched {
                let dict = PyDict::new(py);
                dict.set_item("id", result.template_id)?;
                dict.set_item("name", result.template_name)?;
                dict.set_item("severity", format!("{:?}", result.severity))?;
                dict.set_item("url", result.url)?;
                dict.set_item("evidence", result.evidence)?;
                py_list.append(dict)?;
            }
        }
        
        Ok(py_list.to_object(py))
    }
}

// ==================== Module Definition ====================
#[pymodule]
fn transR(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PipelineR>()?;
    m.add_class::<Feature>()?;
    
    // Add version
    m.add("__version__", "0.1.0")?;
    
    // Add description
    m.add("__doc__", "TransR - Advanced Rust Extension for Python with Pipeline System")?;
    
    Ok(())
}

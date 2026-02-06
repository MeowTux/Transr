use crate::errhuman::{TransRError, TransRResult};
use crate::vvv::Verbose;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type DecoratorFn = Arc<dyn Fn(&mut TaskContext) -> TransRResult<()> + Send + Sync>;

pub struct TaskContext {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl TaskContext {
    pub fn new(name: &str) -> Self {
        TaskContext {
            name: name.to_string(),
            args: HashMap::new(),
            result: None,
            metadata: HashMap::new(),
        }
    }
}

pub struct DecoratorRegistry {
    decorators: Arc<Mutex<HashMap<String, DecoratorFn>>>,
}

impl DecoratorRegistry {
    pub fn new() -> Self {
        let mut registry = DecoratorRegistry {
            decorators: Arc::new(Mutex::new(HashMap::new())),
        };
        
        // Register built-in decorators
        registry.register_builtin();
        registry
    }
    
    fn register_builtin(&mut self) {
        // @timing - measure execution time
        self.register("timing", Arc::new(|ctx| {
            let start = std::time::Instant::now();
            Verbose::info(&format!("⏱️  Starting: {}", ctx.name));
            
            // Store start time in metadata
            ctx.metadata.insert("start_time".to_string(), format!("{:?}", start));
            
            Ok(())
        }));
        
        // @log - log execution
        self.register("log", Arc::new(|ctx| {
            Verbose::info(&format!("📝 Executing: {}", ctx.name));
            Verbose::debug(&format!("   Args: {:?}", ctx.args));
            Ok(())
        }));
        
        // @region - check if feature is in allowed region
        self.register("region", Arc::new(|ctx| {
            let allowed_regions = vec!["global", "asia", "us", "eu"];
            
            if let Some(region) = ctx.args.get("region") {
                if !allowed_regions.contains(&region.as_str()) {
                    return Err(TransRError::RegionError(ctx.name.clone()));
                }
                Verbose::info(&format!("✓ Region check passed: {}", region));
            }
            Ok(())
        }));
        
        // @retry - retry on failure
        self.register("retry", Arc::new(|ctx| {
            let max_retries = ctx.args.get("max_retries")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(3);
            
            ctx.metadata.insert("max_retries".to_string(), max_retries.to_string());
            Verbose::info(&format!("🔄 Retry enabled: max {} attempts", max_retries));
            Ok(())
        }));
        
        // @cache - enable caching
        self.register("cache", Arc::new(|ctx| {
            Verbose::info(&format!("💾 Cache enabled for: {}", ctx.name));
            ctx.metadata.insert("cache_enabled".to_string(), "true".to_string());
            Ok(())
        }));
        
        // @validate - validate inputs
        self.register("validate", Arc::new(|ctx| {
            Verbose::info(&format!("✓ Validating inputs for: {}", ctx.name));
            
            if let Some(required) = ctx.args.get("required") {
                let required_fields: Vec<&str> = required.split(',').collect();
                for field in required_fields {
                    if !ctx.args.contains_key(field.trim()) {
                        return Err(TransRError::ValidationError(
                            format!("Missing required field: {}", field)
                        ));
                    }
                }
            }
            Ok(())
        }));
        
        // @async - mark as async operation
        self.register("async", Arc::new(|ctx| {
            Verbose::info(&format!("⚡ Async mode enabled for: {}", ctx.name));
            ctx.metadata.insert("async".to_string(), "true".to_string());
            Ok(())
        }));
        
        // @secure - add security checks
        self.register("secure", Arc::new(|ctx| {
            Verbose::info(&format!("🔒 Security checks enabled for: {}", ctx.name));
            ctx.metadata.insert("secure".to_string(), "true".to_string());
            Ok(())
        }));
    }
    
    pub fn register(&mut self, name: &str, decorator: DecoratorFn) {
        let mut decorators = self.decorators.lock().unwrap();
        decorators.insert(name.to_string(), decorator);
        Verbose::debug(&format!("Registered decorator: @{}", name));
    }
    
    pub fn apply(&self, decorator_name: &str, ctx: &mut TaskContext) -> TransRResult<()> {
        let decorators = self.decorators.lock().unwrap();
        
        if let Some(decorator) = decorators.get(decorator_name) {
            decorator(ctx)?;
        } else {
            return Err(TransRError::ValidationError(
                format!("Unknown decorator: @{}", decorator_name)
            ));
        }
        
        Ok(())
    }
    
    pub fn list_decorators(&self) -> Vec<String> {
        let decorators = self.decorators.lock().unwrap();
        decorators.keys().cloned().collect()
    }
}

impl Default for DecoratorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to create a task with decorators
pub fn decorated_task(name: &str, decorators: Vec<&str>) -> TransRResult<TaskContext> {
    let registry = DecoratorRegistry::new();
    let mut ctx = TaskContext::new(name);
    
    for decorator in decorators {
        registry.apply(decorator, &mut ctx)?;
    }
    
    Ok(ctx)
}

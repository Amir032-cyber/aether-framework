use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

pub struct Orchestrator {
    modules: Arc<Mutex<HashMap<String, ModuleInfo>>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_module(&self, id: String, name: String, capabilities: Vec<String>) {
        let mut modules = self.modules.lock().unwrap();
        info!("Enregistrement du module: {} ({})", name, id);
        modules.insert(id.clone(), ModuleInfo {
            id,
            name,
            capabilities,
        });
    }

    pub fn list_modules(&self) -> Vec<String> {
        let modules = self.modules.lock().unwrap();
        modules.values().map(|m| m.name.clone()).collect()
    }
}

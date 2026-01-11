use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub mod aether_grpc {
    tonic::include_proto!("aether");
}

pub struct Module {
    pub id: String,
    pub name: String,
    pub endpoint: String,
}

pub struct Orchestrator {
    modules: Arc<RwLock<HashMap<String, Module>>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_module(&self, id: String, name: String, endpoint: String) {
        let mut modules = self.modules.write().await;
        info!("Enregistrement du module: {} ({}) à {}", name, id, endpoint);
        modules.insert(id.clone(), Module { id, name, endpoint });
    }

    pub async fn get_active_modules_count(&self) -> usize {
        let modules = self.modules.read().await;
        modules.len()
    }
}

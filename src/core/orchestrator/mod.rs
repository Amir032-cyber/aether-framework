use anyhow::{bail, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

#[derive(Clone, Default)]
pub struct Orchestrator {
    modules: Arc<Mutex<HashMap<String, ModuleInfo>>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_module(
        &self,
        id: impl Into<String>,
        name: impl Into<String>,
        capabilities: Vec<String>,
    ) -> Result<()> {
        let id = id.into();
        let name = name.into();
        if id.trim().is_empty() || name.trim().is_empty() {
            bail!("l'identifiant et le nom du module sont obligatoires");
        }
        let mut modules = self.modules.lock().expect("orchestrateur verrouillé");
        if modules.contains_key(&id) {
            bail!("le module '{id}' est déjà enregistré");
        }
        info!(module_id = %id, module_name = %name, "Enregistrement du module");
        modules.insert(
            id.clone(),
            ModuleInfo {
                id,
                name,
                capabilities,
            },
        );
        Ok(())
    }

    pub fn unregister_module(&self, id: &str) -> bool {
        self.modules
            .lock()
            .expect("orchestrateur verrouillé")
            .remove(id)
            .is_some()
    }

    pub fn list_modules(&self) -> Vec<ModuleInfo> {
        let mut modules: Vec<_> = self
            .modules
            .lock()
            .expect("orchestrateur verrouillé")
            .values()
            .cloned()
            .collect();
        modules.sort_by(|a, b| a.id.cmp(&b.id));
        modules
    }

    pub fn find_by_capability(&self, capability: &str) -> Vec<ModuleInfo> {
        self.list_modules()
            .into_iter()
            .filter(|module| module.capabilities.iter().any(|item| item == capability))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enregistre_et_filtre_un_module() {
        let orchestrator = Orchestrator::new();
        orchestrator
            .register_module("mapper", "QuantumMapper", vec!["inventory".into()])
            .unwrap();
        assert_eq!(orchestrator.find_by_capability("inventory").len(), 1);
        assert!(orchestrator
            .register_module("mapper", "Duplicate", vec![])
            .is_err());
    }
}

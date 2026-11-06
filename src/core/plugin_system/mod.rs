use anyhow::{bail, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginDescriptor {
    pub id: String,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
}

#[derive(Clone, Default)]
pub struct PluginSystem {
    plugins: Arc<Mutex<HashMap<String, PluginDescriptor>>>,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, descriptor: PluginDescriptor) -> Result<()> {
        if descriptor.id.trim().is_empty()
            || descriptor.name.trim().is_empty()
            || descriptor.version.trim().is_empty()
        {
            bail!("un plugin doit avoir un identifiant, un nom et une version");
        }
        if descriptor
            .capabilities
            .iter()
            .any(|capability| capability.trim().is_empty())
        {
            bail!("les capacités vides sont interdites");
        }
        let mut plugins = self.plugins.lock().expect("registre de plugins verrouillé");
        if plugins.contains_key(&descriptor.id) {
            bail!("le plugin '{}' est déjà enregistré", descriptor.id);
        }
        plugins.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    pub fn unregister(&self, id: &str) -> bool {
        self.plugins
            .lock()
            .expect("registre de plugins verrouillé")
            .remove(id)
            .is_some()
    }

    pub fn list(&self) -> Vec<PluginDescriptor> {
        let mut plugins: Vec<_> = self
            .plugins
            .lock()
            .expect("registre de plugins verrouillé")
            .values()
            .cloned()
            .collect();
        plugins.sort_by(|left, right| left.id.cmp(&right.id));
        plugins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valide_et_dedoublonne_les_plugins() {
        let registry = PluginSystem::new();
        let descriptor = PluginDescriptor {
            id: "inventory".into(),
            name: "Inventory plugin".into(),
            version: "0.1.0".into(),
            capabilities: vec!["inventory.read".into()],
        };
        registry.register(descriptor.clone()).unwrap();
        assert!(registry.register(descriptor).is_err());
        assert_eq!(registry.list().len(), 1);
    }
}

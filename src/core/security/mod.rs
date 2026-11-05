use anyhow::{bail, Result};
use std::collections::HashSet;
use std::net::IpAddr;

#[derive(Clone, Debug, Default)]
pub struct SecurityManager {
    allowed_targets: HashSet<String>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_target(&mut self, target: impl Into<String>) -> Result<()> {
        let target = target.into();
        if target.trim().is_empty() {
            bail!("une cible vide n'est pas autorisée");
        }
        if !Self::is_safe_target(&target) {
            bail!("cible non autorisée: ajoutez-la explicitement à la liste de laboratoire");
        }
        self.allowed_targets.insert(target);
        Ok(())
    }

    pub fn is_allowed(&self, target: &str) -> bool {
        self.allowed_targets.contains(target)
    }

    fn is_safe_target(target: &str) -> bool {
        if matches!(target, "localhost" | "127.0.0.1" | "::1") {
            return true;
        }
        target
            .parse::<IpAddr>()
            .map(|ip| match ip {
                IpAddr::V4(address) => {
                    address.is_loopback() || address.is_private() || address.is_link_local()
                }
                IpAddr::V6(address) => {
                    address.is_loopback()
                        || address.is_unique_local()
                        || address.is_unicast_link_local()
                }
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuse_une_cible_publique() {
        let mut manager = SecurityManager::new();
        assert!(manager.allow_target("example.com").is_err());
        assert!(manager.allow_target("127.0.0.1").is_ok());
        assert!(manager.is_allowed("127.0.0.1"));
    }
}

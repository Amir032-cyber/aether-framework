mod core;
mod utils;

use crate::core::api_gateway::ApiGateway;
use crate::core::data_lake::DataLake;
use crate::core::orchestrator::Orchestrator;
use crate::core::plugin_system::{PluginDescriptor, PluginSystem};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::setup_logger()?;

    let orchestrator = Orchestrator::new();
    orchestrator.register_module("aether-core", "AETHER Core", vec!["health".into()])?;
    let plugins = PluginSystem::new();
    plugins.register(PluginDescriptor {
        id: "aether-core".into(),
        name: "AETHER Core registry".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        capabilities: vec!["health".into(), "inventory.read".into()],
    })?;
    let events = DataLake::new();
    events.append("startup", "aether-core", "service initialisé");
    info!(
        modules = orchestrator.list_modules().len(),
        plugins = plugins.list().len(),
        audit_events = events.len(),
        "AETHER Core initialisé"
    );

    ApiGateway::new_with_orchestrator(8080, orchestrator)
        .start()
        .await
}

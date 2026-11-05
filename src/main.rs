mod core;

use crate::core::api_gateway::ApiGateway;
use crate::core::orchestrator::Orchestrator;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let orchestrator = Orchestrator::new();
    orchestrator.register_module("aether-core", "AETHER Core", vec!["health".into()])?;
    info!(
        modules = orchestrator.list_modules().len(),
        "AETHER Core initialisé"
    );

    ApiGateway::new(8080).start().await
}

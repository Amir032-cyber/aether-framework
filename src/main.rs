mod core;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::core::orchestrator::Orchestrator;
use crate::core::api_gateway::ApiGateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialisation du logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Démarrage de AETHER Core v0.1.0...");
    
    // Initialiser l'orchestrateur
    let orchestrator = Orchestrator::new();
    info!("Orchestrateur initialisé.");
    
    // Démarrer la passerelle API
    let gateway = ApiGateway::new(8080);
    gateway.start().await?;
    
    info!("AETHER Core est prêt.");
    
    // Garder le processus en vie
    tokio::signal::ctrl_c().await?;
    info!("Arrêt de AETHER Core.");
    
    Ok(())
}

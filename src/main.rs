use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialisation du logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Démarrage de AETHER Core v0.1.0...");
    
    // TODO: Initialiser l'orchestrateur
    // TODO: Démarrer la passerelle API
    
    info!("AETHER Core est prêt.");
    
    // Garder le processus en vie
    tokio::signal::ctrl_c().await?;
    info!("Arrêt de AETHER Core.");
    
    Ok(())
}

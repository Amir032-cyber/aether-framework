use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod core;
use crate::core::orchestrator::Orchestrator;
use crate::core::orchestrator::aether_grpc::aether_core_server::AetherCoreServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialisation du logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Démarrage de AETHER Core v0.1.0...");
    
    let addr = "0.0.0.0:50051".parse()?;
    let orchestrator = Orchestrator::new();

    info!("Serveur gRPC AETHER à l'écoute sur {}", addr);

    Server::builder()
        .add_service(AetherCoreServer::new(orchestrator))
        .serve(addr)
        .await?;

    Ok(())
}

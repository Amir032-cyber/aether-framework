use tracing::info;

pub struct ApiGateway {
    port: u16,
}

impl ApiGateway {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Démarrage de la passerelle API sur le port {}...", self.port);
        // Ici, nous pourrions initialiser un serveur HTTP (ex: Axum ou Actix)
        Ok(())
    }
}

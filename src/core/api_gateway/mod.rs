use crate::core::orchestrator::Orchestrator;
use anyhow::Context;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Clone)]
pub struct ApiGateway {
    port: u16,
    orchestrator: Arc<Orchestrator>,
}

impl ApiGateway {
    pub fn new(port: u16) -> Self {
        Self::new_with_orchestrator(port, Orchestrator::new())
    }

    pub fn new_with_orchestrator(port: u16, orchestrator: Orchestrator) -> Self {
        Self {
            port,
            orchestrator: Arc::new(orchestrator),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", self.port))
            .await
            .with_context(|| format!("impossible d'écouter sur 127.0.0.1:{}", self.port))?;
        let address = listener.local_addr()?;
        info!(%address, "Passerelle API démarrée");

        loop {
            let (mut socket, peer) = listener.accept().await?;
            let orchestrator = Arc::clone(&self.orchestrator);
            tokio::spawn(async move {
                let mut buffer = [0_u8; 4096];
                let read = match socket.read(&mut buffer).await {
                    Ok(size) => size,
                    Err(error) => {
                        warn!(%peer, %error, "lecture HTTP échouée");
                        return;
                    }
                };
                let request = String::from_utf8_lossy(&buffer[..read]);
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/");
                let (status, body) = match path {
                    "/" => (
                        "200 OK",
                        r#"{"service":"aether-core","status":"ok"}"#.to_string(),
                    ),
                    "/health" => (
                        "200 OK",
                        r#"{"status":"ok","service":"aether-core"}"#.to_string(),
                    ),
                    "/modules" => match serde_json::to_string(&orchestrator.list_modules()) {
                        Ok(body) => ("200 OK", body),
                        Err(error) => {
                            warn!(%peer, %error, "sérialisation échouée");
                            (
                                "500 Internal Server Error",
                                r#"{"error":"serialization_failed"}"#.to_string(),
                            )
                        }
                    },
                    "/metrics" => (
                        "200 OK",
                        format!(
                            "{{\"modules_registered\":{},\"service\":\"aether-core\"}}",
                            orchestrator.count()
                        ),
                    ),
                    _ => ("404 Not Found", r#"{"error":"not_found"}"#.to_string()),
                };
                let response = format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
                if let Err(error) = socket.write_all(response.as_bytes()).await {
                    warn!(%peer, %error, "écriture HTTP échouée");
                }
            });
        }
    }
}

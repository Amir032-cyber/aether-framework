use anyhow::Context;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{info, warn};

pub struct ApiGateway {
    port: u16,
}

impl ApiGateway {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", self.port))
            .await
            .with_context(|| format!("impossible d'écouter sur 127.0.0.1:{}", self.port))?;
        let address = listener.local_addr()?;
        info!(%address, "Passerelle API démarrée");

        loop {
            let (mut socket, peer) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buffer = [0_u8; 2048];
                let read = match socket.read(&mut buffer).await {
                    Ok(size) => size,
                    Err(error) => {
                        warn!(%peer, %error, "lecture HTTP échouée");
                        return;
                    }
                };
                let request = String::from_utf8_lossy(&buffer[..read]);
                let (status, body) = if request.starts_with("GET /health") {
                    ("200 OK", r#"{"status":"ok","service":"aether-core"}"#)
                } else {
                    ("404 Not Found", r#"{"error":"not_found"}"#)
                };
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                if let Err(error) = socket.write_all(response.as_bytes()).await {
                    warn!(%peer, %error, "écriture HTTP échouée");
                }
            });
        }
    }
}

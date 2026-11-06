use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup_logger() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

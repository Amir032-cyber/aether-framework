use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("kernel-orchestra prêt; aucun programme eBPF n'est chargé automatiquement.");
    println!("Utilisez une configuration explicite et une autorisation documentée avant toute instrumentation.");
    Ok(())
}

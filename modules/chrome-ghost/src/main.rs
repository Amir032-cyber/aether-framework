use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("chrome-ghost prêt; aucune session navigateur n'est lancée automatiquement.");
    println!("Configurez une session autorisée avant d'utiliser les composants d'automatisation.");
    Ok(())
}

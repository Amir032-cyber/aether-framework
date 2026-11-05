use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "aether",
    version,
    about = "CLI de supervision du framework AETHER"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Affiche les informations du cœur local.
    Status,
    /// Vérifie qu’une cible est explicitement déclarée dans l’environnement de travail.
    Scope { target: String },
}

fn main() {
    match Cli::parse().command {
        Command::Status => println!("AETHER CLI prêt; utilisez le cœur local pour les modules."),
        Command::Scope { target } => {
            if target == "localhost" || target.starts_with("127.") || target.starts_with("192.168.")
            {
                println!("Portée locale acceptée: {target}");
            } else {
                eprintln!("Portée refusée par défaut: {target}");
                std::process::exit(2);
            }
        }
    }
}

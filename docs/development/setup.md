# Développement local

## Pré-requis

- Rust stable et Cargo
- `protoc` (Protocol Buffers)
- Python 3.11 ou supérieur pour les modules Python
- Go 1.22 ou supérieur pour les modules Go

## Compiler le cœur

```bash
cargo test --manifest-path src/Cargo.toml
cargo run --manifest-path src/Cargo.toml
```

La passerelle de santé écoute par défaut sur `127.0.0.1:8080` et répond à `GET /health`.

## Compiler le CLI

```bash
cargo run -p aether-cli -- status
cargo run -p aether-cli -- scope localhost
```

Les modules sont séparés afin que chaque intégration soit testée avec ses propres dépendances et permissions. Aucun module ne doit lancer une action sur une cible sans portée explicitement configurée.

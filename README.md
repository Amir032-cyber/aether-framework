# AETHER Framework - Pentesting Unifié

AETHER est un framework de pentesting de nouvelle génération conçu pour l'unification des outils de sécurité avancés. Il utilise une architecture distribuée basée sur gRPC pour permettre une communication fluide entre des modules écrits en Rust, Go et Python.

## 🚀 Installation Rapide

### Prérequis
- Linux (Ubuntu, Kali, Debian, Fedora, Arch)
- Accès Root/Sudo

### Installation Automatique
```bash
git clone https://github.com/Amir032-cyber/aether-framework.git
cd aether-framework
chmod +x scripts/setup.sh
./scripts/setup.sh
```

## 🏗️ Architecture

- **Core (Rust)** : L'orchestrateur central gérant l'enregistrement des modules et le routage des commandes.
- **Modules** :
  - **NEUROSPLOIT (Go)** : IA pour la génération d'exploits.
  - **QUANTUMMAPPER (Python)** : Analyse sémantique et cartographie.
  - **CHROME-GHOST (Rust)** : Automatisation furtive du navigateur.
- **Communication** : gRPC (Protocol Buffers) pour une latence minimale.

## 🛠️ Utilisation

1. **Démarrer le Core** :
   ```bash
   cd src
   cargo run
   ```

2. **Lancer un Module (ex: NeuroSploit)** :
   ```bash
   cd modules/neurospoit
   go run cmd/neurospoit/main.go
   ```

## 📚 Documentation
- [Guide d'installation détaillé](docs/installation.md)
- [Développement de modules](docs/modules/README.md)
- [Spécifications API](docs/api/openapi.yaml)

## 🛡️ Sécurité
Veuillez consulter notre [Politique de Sécurité](SECURITY.md) pour signaler des vulnérabilités.

## 📄 Licence
Ce projet est sous licence Apache-2.0.

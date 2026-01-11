# Guide d'Installation AETHER Framework

Ce guide détaille les étapes pour installer AETHER sur différentes distributions Linux.

## 📋 Dépendances Système

### Debian / Ubuntu / Kali Linux
```bash
sudo apt update
sudo apt install -y build-essential curl git pkg-config libssl-dev protobuf-compiler golang python3 python3-pip
```

### Fedora / RHEL / CentOS
```bash
sudo dnf install -y gcc gcc-c++ make curl git openssl-devel protobuf-compiler golang python3 python3-pip
```

### Arch Linux
```bash
sudo pacman -S --noconfirm base-devel curl git openssl protobuf go python python-pip
```

## 🦀 Installation de Rust
Si Rust n'est pas installé :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## ⚙️ Configuration du Projet

1. **Clonage du dépôt** :
   ```bash
   git clone https://github.com/Amir032-cyber/aether-framework.git
   cd aether-framework
   ```

2. **Initialisation de l'environnement Python** :
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install -r requirements.txt
   ```

3. **Compilation du Core** :
   ```bash
   cd src
   cargo build --release
   ```

## 🐳 Utilisation avec Docker
```bash
docker-compose up --build
```

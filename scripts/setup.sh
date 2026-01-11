#!/bin/bash

set -e

echo "--- AETHER Framework Setup ---"

# Détection de la distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
else
    OS=$(uname -s)
fi

echo "Système détecté : $OS"

install_dependencies() {
    case $OS in
        ubuntu|debian|kali)
            sudo apt-get update
            sudo apt-get install -y build-essential curl git pkg-config libssl-dev protobuf-compiler golang python3 python3-pip
            ;;
        fedora|rhel|centos)
            sudo dnf install -y gcc gcc-c++ make curl git openssl-devel protobuf-compiler golang python3 python3-pip
            ;;
        arch)
            sudo pacman -S --noconfirm base-devel curl git openssl protobuf go python python-pip
            ;;
        *)
            echo "Distribution non supportée automatiquement. Veuillez installer Rust, Go, Python et Protobuf manuellement."
            ;;
    esac
}

install_rust() {
    if ! command -v cargo &> /dev/null; then
        echo "Installation de Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    else
        echo "Rust est déjà installé."
    fi
}

echo "Installation des dépendances système..."
install_dependencies

echo "Vérification de Rust..."
install_rust

echo "Configuration de l'environnement..."
# Création d'un environnement virtuel Python pour les modules
python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip

echo "Installation terminée avec succès !"
echo "Pour démarrer le Core : cd src && cargo run"

"""Validation de portée pour la découverte en environnement de laboratoire.

Ce module ne lance aucune requête réseau. Il fournit uniquement des contrôles
réutilisables par les implémentations de découverte autorisées.
"""
from __future__ import annotations

import ipaddress
import socket


def is_private_target(target: str) -> bool:
    """Retourne True pour localhost, IP privées/lien-locales ou noms locaux."""
    value = target.strip().lower()
    if not value:
        return False
    if value in {"localhost", "localhost.localdomain"}:
        return True
    try:
        address = ipaddress.ip_address(value)
    except ValueError:
        return value.endswith((".local", ".lan", ".internal"))
    return address.is_loopback or address.is_private or address.is_link_local


def resolve_private_target(target: str) -> str:
    """Résout une cible uniquement si elle reste dans une portée privée."""
    if not is_private_target(target):
        raise ValueError("cible publique refusée; utilisez une cible de laboratoire")
    host = target.strip()
    address = socket.gethostbyname(host) if not _looks_like_ip(host) else host
    if not is_private_target(address):
        raise ValueError("la résolution sort de la portée privée")
    return address


def _looks_like_ip(value: str) -> bool:
    try:
        ipaddress.ip_address(value)
        return True
    except ValueError:
        return False

# Déploiement

Le déploiement recommandé commence par le cœur local en écoute sur `127.0.0.1:8080`. Pour un environnement partagé, placer un proxy TLS devant le service, limiter les entrées réseau et conserver `readOnlyRootFilesystem` ainsi que `allowPrivilegeEscalation: false`.

```bash
docker compose -f deployments/docker/docker-compose.yml up --build
```

Les modules expérimentaux ne doivent pas être déployés sans revue de sécurité et portée écrite.

## Laboratoire Docker isolé

Le laboratoire est séparé du déploiement normal et utilise uniquement des cibles de formation
volontairement vulnérables : OWASP Juice Shop, WebGoat/WebWolf et DVWA. Tous les ports sont
publiés sur `127.0.0.1` uniquement. Le réseau `aether-lab-isolated` est marqué `internal: true`,
les conteneurs n’ont pas de privilèges supplémentaires, et chaque cible possède des limites de
CPU, mémoire et processus.

Docker et Docker Compose sont requis. Depuis la racine du dépôt :

```bash
./scripts/lab-up.sh
./scripts/lab-smoke.sh
```

Adresses locales :

| Cible | URL |
| --- | --- |
| Juice Shop | http://127.0.0.1:3000 |
| WebGoat | http://127.0.0.1:8081/WebGoat |
| WebWolf | http://127.0.0.1:9090/WebWolf |
| DVWA | http://127.0.0.1:8082 |
| Aether Core | http://127.0.0.1:8080/health |

Pour arrêter et supprimer le laboratoire :

```bash
./scripts/lab-down.sh
```

Ne publiez jamais ces ports sur une interface réseau publique et n’utilisez pas ces images
contre un système tiers. Les tests doivent rester dans le réseau Docker dédié et respecter la
politique de sécurité du projet.

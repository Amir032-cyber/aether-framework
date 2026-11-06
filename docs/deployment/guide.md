# Déploiement

Le déploiement recommandé commence par le cœur local en écoute sur `127.0.0.1:8080`. Pour un environnement partagé, placer un proxy TLS devant le service, limiter les entrées réseau et conserver `readOnlyRootFilesystem` ainsi que `allowPrivilegeEscalation: false`.

```bash
docker compose -f deployments/docker/docker-compose.yml up --build
```

Les modules expérimentaux ne doivent pas être déployés sans revue de sécurité et portée écrite.

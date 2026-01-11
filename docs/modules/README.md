# Développement de Modules pour AETHER

AETHER supporte les modules écrits dans n'importe quel langage supportant gRPC (Rust, Go, Python, C++, etc.).

## 🛠️ Étapes pour créer un module

1. **Définir l'interface** : Utilisez le fichier `src/proto/modules.proto` pour définir les services de votre module.
2. **Implémenter le serveur gRPC** : Votre module doit agir comme un serveur gRPC écoutant sur un port spécifique.
3. **S'enregistrer auprès du Core** : Au démarrage, le module doit appeler la méthode `RegisterModule` du Core AETHER (port 50051 par défaut).

## 📋 Exemple en Python

```python
import grpc
import aether_pb2
import aether_pb2_grpc

def register():
    channel = grpc.insecure_channel('localhost:50051')
    stub = aether_pb2_grpc.AetherCoreStub(channel)
    response = stub.RegisterModule(aether_pb2.ModuleInfo(
        id="mon-module",
        name="Mon Module de Test",
        endpoint="localhost:50055"
    ))
    print(response.message)
```

## 🔍 Santé du Module
Chaque module doit implémenter un check de santé via `GetHealth` pour permettre à l'orchestrateur de surveiller son état.

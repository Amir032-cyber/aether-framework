# Politique de sécurité AETHER

AETHER est destiné exclusivement aux audits autorisés, aux environnements de laboratoire et aux systèmes dont l’utilisateur possède l’autorisation écrite. Le projet ne doit pas être utilisé pour contourner une authentification, exfiltrer des données, persister sur un système tiers ou perturber un service.

## Règles minimales

1. Définir une portée écrite avant chaque test : cibles, horaires, méthodes autorisées et contact d’urgence.
2. Utiliser en priorité des cibles locales, privées ou de laboratoire. Les modules doivent refuser par défaut les cibles publiques non explicitement autorisées.
3. Ne jamais stocker de secrets, jetons, données personnelles ou résultats sensibles dans Git.
4. Journaliser les actions importantes sans enregistrer de contenu confidentiel.
5. Arrêter immédiatement un test en cas d’impact inattendu et conserver les preuves de manière sécurisée.
6. Corriger ou supprimer toute capacité qui facilite une intrusion non autorisée, une persistance furtive ou une exfiltration.

## Statut du dépôt

Le cœur AETHER fournit actuellement une orchestration de modules, une sonde HTTP locale `/health` et une validation de cibles de laboratoire. Les modules offensifs annoncés dans l’architecture restent expérimentaux et ne doivent pas être considérés comme prêts pour la production.

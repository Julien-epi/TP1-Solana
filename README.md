# 🎮 RPG sur la blockchain Solana

Ce projet implémente un petit jeu de rôle (RPG) sur la blockchain Solana en utilisant le framework Anchor. Chaque joueur est représenté par un compte blockchain contenant des informations sur son personnage.

## 📝 Structure du Projet

Le programme Solana permet de:
- Créer un compte joueur avec un pseudonyme
- Gérer les attributs du joueur (points de vie, expérience, niveau)
- Acheter de l'expérience en échange de SOL
- Combattre d'autres joueurs
- Système de niveaux basé sur l'XP

## 🚀 Déploiement

Le programme a été déployé avec succès sur le réseau Solana devnet le 3 mai 2025.
- **ID du Programme**: 4RgzWS9Gixt44wwULLUVw47Dixxh1ywbGkZ4D1yPVUgn
- **Wallet Déployeur**: FwN1nDBaVhEzjnYRN537zx4hx3FgXRiGfRUKTf1ywCib

## 📊 Système de Niveaux

Le programme implémente 4 niveaux de joueur basés sur l'expérience:
- **Beginner**: XP < 5
- **Explorer**: 5 ≤ XP < 35
- **Champion**: 35 ≤ XP < 100
- **Legend**: XP ≥ 100

## 📊 Résumé des Tests

| Action               | Status | Signature Transaction                |
|----------------------|--------|-------------------------------------|
| Création Joueur (V1) | ✅     | 5gJe7ppY8oLsQN7bRSjuqmS2ApdPTGPYHL6p2GkBFgpdz9euYsGGWPGaBTBS1abVHd3Sh9pa4jqixjMMLUDNaBFu |
| Achat XP (V2)        | ✅     | 3AeELAkGCqK6gesyXw4xGfwdhTBJATZVNq8Tm5cuUHSMksqJUSHPhkAyALgYCK1PnHQrFspxutASVGhRVrRTieoa|                     |
| Retrait Vault (V3)   | ✅     | 3gS5k19KN7f8XJ9Bs2nVHPYyxNabvAkwfYnfZpWXG838tLgBpio3NVytW6qjH615wYyQh5Y475dAFC9zac1u9hp6 |
| Combat (V4)          |      |                                    |



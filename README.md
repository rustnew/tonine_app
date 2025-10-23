# 🎬 Backend Tontine - Documentation Complète

## 📋 **Introduction**

**Backend Tontine** est une API moderne et robuste développée en **Rust** pour gérer des systèmes de tontines digitales, particulièrement adaptée au contexte camerounais et africain.

### 🎯 **Problème Résolu**
- Digitaliser les tontines traditionnelles très populaires en Afrique
- Offrir transparence et traçabilité des transactions
- Automatiser la gestion des cotisations et des bénéficiaires
- Sécuriser les épargnes collectives

## 🏗️ **Architecture Technique**

### **Stack Technologique**
- **🦀 Langage**: Rust (performance, sécurité mémoire)
- **🌐 Framework Web**: Actix-web (concurrent, haute performance)
- **🗄️ Base de Données**: PostgreSQL (relations, transactions ACID)
- **📊 ORM/Query Builder**: SQLx (compile-time safety)
- **🔐 Sécurité**: Bcrypt (hash passwords), UUIDv4

### **Structure du Projet**
```
backend/
├── src/
│   ├── auth/           # Authentification et autorisation
│   ├── handlers/       # Logique métier des endpoints
│   ├── routes/         # Définition des routes API
│   ├── repositories/   # Accès aux données
│   ├── model/          # Structures de données
│   ├── errors.rs       # Gestion centralisée des erreurs
│   └── main.rs         # Point d'entrée de l'application
├── migrations/         # Scripts de migration SQL
├── .env               # Variables d'environnement
├── Cargo.toml         # Dépendances Rust
└── README.md          # Documentation
```

## 📊 **Modèle de Données**

### **6 Tables Principales**
1. **👥 Users** - Gestion des membres
2. **🏦 Tontines** - Configuration des tontines
3. **🤝 Tontine Members** - Adhésions aux tontines
4. **🔄 Tontine Rounds** - Tours de perception
5. **💰 Contributions** - Paiements des cotisations
6. **📈 Transactions** - Historique complet

## 🚀 **Installation et Démarrage**

### **Prérequis**
- Rust 1.70+
- PostgreSQL 13+
- Cargo (gestionnaire de paquets Rust)

### **📥 Installation de Rust**

```bash
# Installation de Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Recharger le profil shell
source ~/.bashrc  # ou source ~/.zshrc

# Vérification de l'installation
cargo --version
rustc --version
```

### **📦 Installation de SQLx CLI**

```bash
# Installation de l'outil en ligne de commande SQLx
cargo install sqlx-cli

# Vérification
sqlx --version
```

### **🔧 Configuration du Projet**

1. **Cloner le projet**
```bash
git clone <votre-repo>
cd backend-tontine
```

2. **Configurer la base de données**
```bash
# Créer le fichier .env
echo "DATABASE_URL=postgres://username:password@localhost/tontine_db" > .env

# Créer la base de données
sqlx database create

# Exécuter les migrations
sqlx migrate run
```

3. **Installer les dépendances**
```bash
cargo build
```

### **🎯 Démarrage de l'Application**

```bash
# Mode développement (avec rechargement automatique)
cargo run

# Mode production
cargo build --release
./target/release/backend-tontine
```

### **🧪 Tests**

```bash
# Lancer tous les tests
cargo test

# Tests avec output détaillé
cargo test -- --nocapture
```

## 🔌 **API REST - Documentation Complète**

## 🔐 **MODULE D'AUTHENTIFICATION** (`/api/auth`)

### **Routes Publiques** (sans authentification)
| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **POST** | `/api/auth/login` | Connexion d'un utilisateur |
| **POST** | `/api/auth/logout` | Déconnexion d'un utilisateur |
| **POST** | `/api/auth/request-password-reset` | Demande de réinitialisation de mot de passe |
| **POST** | `/api/auth/confirm-password-reset` | Confirmation de réinitialisation de mot de passe |

### **Routes Protégées** (avec authentification Bearer)
| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/auth/me` | Récupérer les informations de l'utilisateur connecté |
| **PUT** | `/api/auth/change-password` | Changer le mot de passe de l'utilisateur connecté |
| **POST** | `/api/auth/refresh-token` | Rafraîchir le token d'authentification |

---

## 👥 **MODULE DES UTILISATEURS** (`/api/users`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/users` | Récupérer la liste de tous les utilisateurs |
| **POST** | `/api/users` | Créer un nouvel utilisateur |
| **GET** | `/api/users/{id}` | Récupérer un utilisateur spécifique par son ID |
| **PUT** | `/api/users/{id}` | Mettre à jour un utilisateur spécifique |
| **DELETE** | `/api/users/{id}` | Supprimer un utilisateur |
| **PUT** | `/api/users/{id}/change-password` | Changer le mot de passe d'un utilisateur |

---

## 💰 **MODULE DES TONTINES** (`/api/tontines`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/tontines` | Récupérer toutes les tontines |
| **POST** | `/api/tontines` | Créer une nouvelle tontine |
| **GET** | `/api/tontines/active` | Récupérer les tontines actives |
| **GET** | `/api/tontines/user/{user_id}` | Récupérer les tontines d'un utilisateur |
| **GET** | `/api/tontines/{id}` | Récupérer une tontine spécifique |
| **GET** | `/api/tontines/{id}/details` | Récupérer une tontine avec les détails du créateur |
| **PUT** | `/api/tontines/{id}` | Mettre à jour une tontine |
| **DELETE** | `/api/tontines/{id}` | Supprimer une tontine |
| **PUT** | `/api/tontines/{id}/increment-round` | Incrémenter le round d'une tontine |

---

## 🔄 **MODULE DES ROUNDS DE TONTINE** (`/api/tontine-rounds`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/tontine-rounds` | Récupérer tous les rounds |
| **POST** | `/api/tontine-rounds` | Créer un nouveau round |
| **GET** | `/api/tontine-rounds/status/{status}` | Récupérer les rounds par statut |
| **GET** | `/api/tontine-rounds/tontine/{tontine_id}` | Récupérer les rounds d'une tontine |
| **GET** | `/api/tontine-rounds/tontine/{tontine_id}/current` | Récupérer le round actuel d'une tontine |
| **GET** | `/api/tontine-rounds/tontine/{tontine_id}/next-round` | Récupérer le numéro du prochain round |
| **GET** | `/api/tontine-rounds/{id}` | Récupérer un round spécifique |
| **PUT** | `/api/tontine-rounds/{id}` | Mettre à jour un round |
| **DELETE** | `/api/tontine-rounds/{id}` | Supprimer un round |
| **PUT** | `/api/tontine-rounds/{id}/complete` | Marquer un round comme complété |
| **PUT** | `/api/tontine-rounds/{id}/cancel` | Annuler un round |

---

## 👤 **MODULE DES MEMBRES DE TONTINE** (`/api/tontine-members`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/tontine-members` | Récupérer tous les membres |
| **POST** | `/api/tontine-members` | Créer un nouveau membre |
| **GET** | `/api/tontine-members/tontine/{tontine_id}` | Récupérer les membres d'une tontine |
| **GET** | `/api/tontine-members/tontine/{tontine_id}/count` | Récupérer le nombre de membres d'une tontine |
| **GET** | `/api/tontine-members/user/{user_id}` | Récupérer les tontines d'un utilisateur |
| **GET** | `/api/tontine-members/{id}` | Récupérer un membre spécifique |
| **PUT** | `/api/tontine-members/{id}` | Mettre à jour un membre |
| **DELETE** | `/api/tontine-members/{id}` | Supprimer un membre |
| **PUT** | `/api/tontine-members/{id}/deactivate` | Désactiver un membre |

---

## 💵 **MODULE DES CONTRIBUTIONS** (`/api/contributions`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/contributions` | Récupérer toutes les contributions |
| **POST** | `/api/contributions` | Créer une nouvelle contribution |
| **GET** | `/api/contributions/round/{round_id}` | Récupérer les contributions d'un round |
| **GET** | `/api/contributions/round/{round_id}/summary` | Récupérer le résumé des contributions d'un round |
| **GET** | `/api/contributions/member/{member_id}` | Récupérer les contributions d'un membre |
| **GET** | `/api/contributions/member/{member_id}/summary` | Récupérer le résumé des contributions d'un membre |
| **GET** | `/api/contributions/{id}` | Récupérer une contribution spécifique |
| **PUT** | `/api/contributions/{id}` | Mettre à jour une contribution |
| **DELETE** | `/api/contributions/{id}` | Supprimer une contribution |
| **PUT** | `/api/contributions/{id}/mark-paid` | Marquer une contribution comme payée |
| **PUT** | `/api/contributions/{id}/mark-failed` | Marquer une contribution comme échouée |

---

## 💳 **MODULE DES TRANSACTIONS** (`/api/transactions`)

| Méthode | Endpoint | Rôle |
|---------|----------|------|
| **GET** | `/api/transactions` | Récupérer toutes les transactions |
| **POST** | `/api/transactions` | Créer une nouvelle transaction |
| **POST** | `/api/transactions/contribution` | Créer une transaction de contribution |
| **POST** | `/api/transactions/payout` | Créer une transaction de paiement |
| **GET** | `/api/transactions/type/{transaction_type}` | Récupérer les transactions par type |
| **GET** | `/api/transactions/tontine/{tontine_id}` | Récupérer les transactions d'une tontine |
| **GET** | `/api/transactions/tontine/{tontine_id}/summary` | Récupérer le résumé financier d'une tontine |
| **GET** | `/api/transactions/user/{user_id}` | Récupérer les transactions d'un utilisateur |
| **GET** | `/api/transactions/user/{user_id}/summary` | Récupérer le résumé financier d'un utilisateur |
| **GET** | `/api/transactions/{id}` | Récupérer une transaction spécifique |
| **PUT** | `/api/transactions/{id}/status/{status}` | Mettre à jour le statut d'une transaction |

---

## 🛡️ **Sécurité et Validation**

### **Mesures de Sécurité**
- 🔒 Hashage bcrypt pour les mots de passe
- 🆔 UUID pour éviter l'énumération
- ✅ Validation des données d'entrée
- 🚨 Gestion d'erreurs structurée
- 💾 Prévention des doublons (email, téléphone)

## ⚡ **Performance et Robustesse**

### **Avantages Rust**
- 🚀 Performances natives (pas de GC)
- 🧬 Sécurité mémoire à la compilation
- 📦 Gestion efficace de la concurrence
- 🔧 Compilation statique

## 🌍 **Contexte Africain et Camerounais**

### **Adaptations Spécifiques**
- 📱 Support Mobile Money dans le modèle
- 💰 Montants en FCFA
- 🔄 Flexibilité des fréquences de cotisation
- 👥 Gestion des groupes (famille, amis, collègues)

## 🔄 **Workflow Typique d'Utilisation**

1. **Inscription** → Création du profil utilisateur
2. **Création Tontine** → Configuration des paramètres
3. **Invitation Membres** → Adhésion des participants
4. **Démarrage Rounds** → Lancement des tours
5. **Cotisations** → Paiements réguliers
6. **Attribution** → Sélection des bénéficiaires
7. **Paiement** → Versement au bénéficiaire
8. **Suivi** → Monitoring des transactions

## 📊 **Variables d'Environnement**

Créez un fichier `.env` à la racine du projet :

```env
DATABASE_URL=postgres://username:password@localhost/tontine_db
JWT_SECRET=votre_secret_jwt_tres_long_ici
PORT=8080
RUST_LOG=debug
```

## 🐛 **Dépannage**

### **Problèmes Courants**

1. **Erreur de connexion à la base de données**
```bash
# Vérifier que PostgreSQL est démarré
sudo systemctl status postgresql

# Vérifier la connexion
psql -U username -d tontine_db
```

2. **Erreur de migration SQLx**
```bash
# Nettoyer et réessayer
sqlx database drop
sqlx database create
sqlx migrate run
```

3. **Problèmes de dépendances Rust**
```bash
# Nettoyer le cache
cargo clean
cargo build
```

## 📞 **Support**

Pour toute question ou problème :
1. Vérifier les logs avec `RUST_LOG=debug cargo run`
2. Consulter la documentation SQLx
3. Vérifier la configuration de la base de données

## 🎯 **Prochaines Étapes**

- [ ] Configurer les variables d'environnement
- [ ] Créer la base de données PostgreSQL
- [ ] Exécuter les migrations
- [ ] Tester les endpoints API
- [ ] Configurer pour la production

---

**🚀 PRÊT POUR LA PROCHAINE RÉVOLUTION FINTECH EN AFRIQUE!** 🚀

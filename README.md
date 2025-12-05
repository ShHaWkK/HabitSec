## Security UX Layer – Plateforme de gamification de la sécurité

Ce projet a pour but de créer une **couche UX entre les politiques de sécurité techniques** (AD / Azure AD / Google Workspace / Okta, GPO, policies PDF, etc.) et les **utilisateurs finaux** (employés non techniques).

La plateforme :
- transforme des signaux techniques en **missions concrètes et gamifiées** pour les employés ;
- calcule un **score de conformité humaine** par utilisateur, équipe et organisation ;
- offre aux admins un **dashboard** pour suivre l’avancement et les risques.

### Stack principale

- **Backend** : Rust, Axum, PostgreSQL, SQLx, API REST/JSON, auth OIDC + mode local.
- **Frontend** : Vue 3, TypeScript, Vite, SPA.
- **Infra & config** : Docker (docker-compose), variables d’environnement, logs structurés JSON.

### Structure du repo

- `backend/` : service API Rust (Axum + SQLx), moteur de règles, connecteurs, scoring.
- `frontend/` : SPA Vue 3 + TS, vues employé/admin, intégration OIDC.
- `docker-compose.yml` : exemple de composition (PostgreSQL, backend, frontend).

Voir les fichiers `backend/README.md` et `frontend/README.md` (à créer/compléter) pour les détails spécifiques.



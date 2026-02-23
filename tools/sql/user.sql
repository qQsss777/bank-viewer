-- Crée le schéma users si nécessaire
CREATE SCHEMA IF NOT EXISTS "users";

-- Définit le search_path pour utiliser le schéma `users` par défaut
SET search_path TO "users";

-- Crée la table dans le schéma users
CREATE TABLE IF NOT EXISTS "users" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    firstname VARCHAR(100),
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insère les données avec une protection contre les conflits de nom d'utilisateur
INSERT INTO "users" (username, password, firstname, name)
VALUES ('foo', 'bar', 'John', 'Doe')
ON CONFLICT (username) DO NOTHING;

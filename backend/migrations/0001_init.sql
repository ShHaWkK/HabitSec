-- Types énumérés pour les utilisateurs et les tâches

CREATE TYPE user_source AS ENUM ('azuread', 'googleworkspace', 'okta', 'local');

CREATE TYPE user_role AS ENUM ('employee', 'admin');

CREATE TYPE task_status AS ENUM ('todo', 'inprogress', 'done');

CREATE TYPE task_type AS ENUM ('mfa', 'device', 'email', 'adminhygiene', 'other');

-- Table des équipes / départements

CREATE TABLE teams (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    average_score INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table des utilisateurs

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    department_id UUID NULL REFERENCES teams(id),
    role user_role NOT NULL,
    source user_source NOT NULL,
    compliance_score INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Lien N-N entre users et teams

CREATE TABLE team_members (
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, user_id)
);

-- Table des tâches gamifiées

CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    task_type task_type NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status task_status NOT NULL,
    points INT NOT NULL,
    due_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);

-- Table des règles

CREATE TABLE rules (
    id UUID PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    condition TEXT NOT NULL,
    task_title_template TEXT NOT NULL,
    task_description_template TEXT NOT NULL,
    points INT NOT NULL,
    task_type TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

-- Historique des scores utilisateurs

CREATE TABLE user_score_snapshots (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    score INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_user_score_snapshots_user_id ON user_score_snapshots(user_id);



-- Données de démo pour tester rapidement l’enchaînement backend + frontend.

INSERT INTO teams (id, name, description, average_score)
VALUES (
  '00000000-0000-0000-0000-00000000aaaa',
  'Equipe Sécurité Demo',
  'Equipe de démonstration pour le POC Security UX Layer',
  42
)
ON CONFLICT (id) DO NOTHING;

INSERT INTO users (id, email, display_name, department_id, role, source, compliance_score)
VALUES (
  '00000000-0000-0000-0000-000000000001',
  'demo.user@example.com',
  'Demo User',
  '00000000-0000-0000-0000-00000000aaaa',
  'employee',
  'local',
  42
)
ON CONFLICT (id) DO NOTHING;

INSERT INTO team_members (team_id, user_id)
VALUES (
  '00000000-0000-0000-0000-00000000aaaa',
  '00000000-0000-0000-0000-000000000001'
)
ON CONFLICT (team_id, user_id) DO NOTHING;

INSERT INTO tasks (
  id,
  user_id,
  task_type,
  title,
  description,
  status,
  points,
  due_date
)
VALUES
(
  '00000000-0000-0000-0000-00000000b001',
  '00000000-0000-0000-0000-000000000001',
  'mfa',
  'Active le 2FA sur ton compte principal',
  'Cette étape renforce fortement la sécurité de ton compte.',
  'todo',
  50,
  now() + interval '3 days'
),
(
  '00000000-0000-0000-0000-00000000b002',
  '00000000-0000-0000-0000-000000000001',
  'device',
  'Vérifie le chiffrement de ton poste',
  'Assure-toi que le disque de ton ordinateur est chiffré.',
  'inprogress',
  30,
  null
)
ON CONFLICT (id) DO NOTHING;



create table users (
  id uuid primary key default gen_random_uuid(),
  username varchar(24) not null unique check(length(username) >= 2 and length(username) <= 24 and username ~ '^[a-zA-Z]([_]?[a-zA-Z0-9])+$'),
  email varchar(255) not null check(email ~ '[^@]+@[^@]+\.[^@]+'),
  password_hash text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

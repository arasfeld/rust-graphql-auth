create table public.users (
  id uuid primary key default uuid_generate_v4(),
  username varchar(24) not null unique,
  email varchar(255) not null unique,
  password_hash text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

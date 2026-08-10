create table activity (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    name text not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

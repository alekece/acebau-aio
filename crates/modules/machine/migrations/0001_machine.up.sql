create type machine_state as enum ('available', 'running', 'maintenance', 'broken');

create table machine_model (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    brand text not null,
    name text not null,
    maintenance_cost numeric not null check (maintenance_cost >= 0),
    lifetime numeric not null check (lifetime > 0),
    average_power numeric not null check (average_power >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (brand, name)
);

create table machine (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    model_id uuid not null references machine_model(id),
    surname text not null,
    purchase_cost numeric not null check (purchase_cost >= 0),
    printing_time numeric not null default 0 check (printing_time >= 0),
    state machine_state not null default 'available',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index machine_model_id_idx on machine(model_id);

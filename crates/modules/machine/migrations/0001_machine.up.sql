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
    nozzle_size numeric not null check (nozzle_size > 0),
    printing_time numeric not null default 0 check (printing_time >= 0),
    state machine_state not null default 'available',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index machine_model_id_idx on machine(model_id);

alter table machine_model
    add column has_carbon_filter boolean not null default false;

create type machine_maintenance_kind as enum (
    'nozzle',
    'axis_cleaning',
    'axis_lubrication',
    'general_cleaning',
    'carbon_filter'
);

create table machine_maintenance_setting (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    kind machine_maintenance_kind not null unique,
    due_after numeric not null check (due_after > 0),
    critical_after numeric not null check (critical_after >= due_after),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

-- Time metrics are persisted in their canonical unit: minutes.
insert into machine_maintenance_setting (kind, due_after, critical_after) values
    ('nozzle', 30000, 60000),
    ('axis_cleaning', 6000, 6000),
    ('axis_lubrication', 60000, 66000),
    ('general_cleaning', 21000, 24000),
    ('carbon_filter', 21000, 66000);

create table machine_maintenance (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    machine_id uuid not null references machine(id) on delete cascade,
    kind machine_maintenance_kind not null,
    performed_at timestamptz not null default now(),
    printing_time numeric not null check (printing_time >= 0),
    notes text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index machine_maintenance_machine_kind_idx
    on machine_maintenance(machine_id, kind, performed_at desc);

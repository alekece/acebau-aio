create table machine_models
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text not null,
    amortized_lifetime interval not null,
    average_energy_consumption numeric not null,
    price numeric not null,
    additional_pieces numeric not null,
    annual_maintenance numeric not null,
    print_width numeric not null,
    print_depth numeric not null,
    print_height numeric not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table machines
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    machine_model_id uuid not null references machine_models(id) on delete cascade,
    printing_environment_id uuid not null references printing_environments(id) on delete cascade,
    nickname text not null,
    purchase_date date not null,
    total_printing_duration interval,
    last_maintenance_at date,
    cost_buffer_factor numeric not null default 1.3 check (cost_buffer_factor >= 1),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('machines');
select trigger_updated_at('machine_models');

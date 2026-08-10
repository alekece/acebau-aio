create table materials
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text unique not null,
    type text not null,
    unit text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table material_providers
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    material_id uuid references materials(id) on delete cascade,
    provider_name text not null,
    url text not null,
    bundle_size numeric not null,
    unit_price numeric not null,
    bulk_price numeric,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('materials');
select trigger_updated_at('material_providers');

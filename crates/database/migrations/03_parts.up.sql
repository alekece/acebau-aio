create table parts
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text unique not null,
    size text not null default 'regular',
    width numeric not null,
    depth numeric not null,
    height numeric not null,
    print_duration numeric not null,
    filament_required numeric not null,
    complexity_factor numeric(3, 1) not null default 1.0,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('parts');

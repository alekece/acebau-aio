create type supply_kind as enum ('filament', 'production_material', 'product_packaging', 'shipping_packaging');
create type spool_state as enum ('sealed', 'open', 'empty', 'discarded');

create table supply (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    name text not null,
    reference text not null unique,
    kind supply_kind not null,
    base_unit text not null,
    available_quantity double precision not null default 0 check (available_quantity >= 0),
    low_stock_threshold double precision not null default 0 check (low_stock_threshold >= 0),
    target_quantity double precision not null default 0 check (target_quantity >= low_stock_threshold),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table filament_spool (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    supply_id uuid not null references supply(id),
    internal_reference text not null unique,
    initial_weight numeric not null check (initial_weight > 0),
    remaining_weight numeric not null check (remaining_weight >= 0 and remaining_weight <= initial_weight),
    received_cost numeric not null check (received_cost >= 0),
    state spool_state not null default 'sealed',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index filament_spool_supply_id_idx on filament_spool(supply_id);

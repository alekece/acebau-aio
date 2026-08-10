create type order_source as enum ('reseller_catalogue', 'direct', 'shopify', 'etsy');
create type order_state as enum ('pending', 'accepted', 'rejected', 'in_production', 'ready_to_ship', 'awaiting_payment', 'completed', 'cancelled');

create table customer_order (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    reference text not null unique,
    reseller_id uuid references reseller(id),
    customer_name text not null,
    source order_source not null,
    state order_state not null default 'pending',
    requested_on date not null,
    total_ht numeric not null check (total_ht >= 0),
    progress_summary text not null default '',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table order_line (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    order_id uuid not null references customer_order(id),
    variant_id uuid not null references variant(id),
    quantity integer not null check (quantity > 0),
    unit_price_ht numeric not null check (unit_price_ht >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index customer_order_reseller_id_idx on customer_order(reseller_id);
create index order_line_order_id_idx on order_line(order_id);
create index order_line_variant_id_idx on order_line(variant_id);

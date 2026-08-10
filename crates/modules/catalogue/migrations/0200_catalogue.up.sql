create table product (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    name text not null,
    collection text not null,
    category text not null,
    short_description text not null default '',
    customizable boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (collection, category, name)
);

create table variant (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    product_id uuid not null references product(id),
    display_name text not null,
    sku text not null unique,
    retail_price numeric not null check (retail_price >= 0),
    reseller_price numeric not null check (reseller_price >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (product_id, display_name)
);

create index variant_product_id_idx on variant(product_id);

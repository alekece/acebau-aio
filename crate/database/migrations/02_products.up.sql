create table products
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text not null,
    description text,
    version integer not null default 1,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table product_variants (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    product_id uuid not null references products(id) on delete cascade,
    sku text unique not null,
    display_name text,
    price_ht numeric(10, 2) not null,
    vat_ratio numeric(4, 3) not null default 0.2,
    resale_coefficient numeric(3, 2) not null,
    height integer,
    width integer,
    length integer,
    weight integer,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('products');
select trigger_updated_at('product_variants');

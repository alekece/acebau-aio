create type reseller_relationship as enum ('prospect', 'approved', 'paused', 'closed', 'rejected');

create table reseller (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    business_name text not null unique,
    city text not null,
    country text not null,
    relationship reseller_relationship not null default 'prospect',
    primary_email text not null,
    next_action_date date,
    next_action text not null default '',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

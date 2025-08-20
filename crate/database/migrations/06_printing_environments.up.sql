create table printing_environments
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text not null unique,
    operating_factor float4 not null default 0.7 check (operating_factor > 0 and operating_factor <= 1),
    electricity_cost_per_kwh float4 not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

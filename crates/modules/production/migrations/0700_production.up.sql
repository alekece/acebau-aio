create type production_state as enum ('to_plan', 'to_start', 'in_progress', 'completed', 'failed', 'cancelled');
create type planning_preference as enum ('quality', 'cost', 'speed');

create table production_task (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    reference text not null unique,
    variant_id uuid not null references variant(id),
    quantity integer not null check (quantity > 0),
    linked_order_reference text,
    deadline date not null,
    state production_state not null default 'to_plan',
    planning_preference planning_preference not null default 'quality',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index production_task_variant_id_idx on production_task(variant_id);

create sequence production_reference_seq;

select setval(
    'production_reference_seq',
    greatest(
        coalesce(max(nullif(regexp_replace(reference, '[^0-9]', '', 'g'), '')::bigint), 0) + 1,
        1
    ),
    false
)
from production_task;

alter table production_task
    alter column reference set default (
        'PRD-' || lpad(nextval('production_reference_seq')::text, 6, '0')
    );
alter table production_task rename column variant_id to legacy_variant_id;
alter table production_task rename column quantity to legacy_quantity;
alter table production_task alter column legacy_variant_id drop not null;
alter table production_task alter column legacy_quantity drop not null;
alter table production_task rename constraint production_task_variant_id_fkey to production_task_legacy_variant_id_fkey;
alter index production_task_variant_id_idx rename to production_task_legacy_variant_id_idx;

create table production_task_line (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    production_task_id uuid not null references production_task(id) on delete cascade,
    piece_id uuid not null references printed_piece(id),
    filament_supply_id uuid not null references supply(id),
    quantity integer not null check (quantity > 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

insert into production_task_line (production_task_id, piece_id, filament_supply_id, quantity)
select
    task.id,
    recipe.piece_id,
    recipe.filament_supply_id,
    task.legacy_quantity * recipe.quantity
from production_task as task
join recipe_item as recipe on recipe.variant_id = task.legacy_variant_id
where recipe.status = 'active';

create index production_task_line_task_id_idx on production_task_line(production_task_id);
create index production_task_line_piece_id_idx on production_task_line(piece_id);
create index production_task_line_filament_supply_id_idx on production_task_line(filament_supply_id);

create table recipe_item (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    variant_id uuid not null references variant(id),
    piece_id uuid not null references printed_piece(id),
    filament_supply_id uuid not null references supply(id),
    quantity integer not null check (quantity > 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (variant_id, piece_id, filament_supply_id)
);

create index recipe_item_piece_id_idx on recipe_item(piece_id);
create index recipe_item_filament_supply_id_idx on recipe_item(filament_supply_id);

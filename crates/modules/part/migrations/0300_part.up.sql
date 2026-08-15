create table printed_piece (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    name text not null,
    reference text not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table piece_machine_profile (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    piece_id uuid not null references printed_piece(id),
    machine_model_id uuid not null references machine_model(id),
    nozzle_size numeric not null check (nozzle_size > 0),
    printing_time numeric not null check (printing_time > 0),
    filament_mass numeric not null check (filament_mass > 0),
    plate_capacity integer not null check (plate_capacity > 0),
    quality_note text not null default '',
    preferred boolean not null default false,
    excluded boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (piece_id, machine_model_id, nozzle_size),
    check (not (preferred and excluded))
);

create unique index one_preferred_machine_profile_per_piece_idx
    on piece_machine_profile(piece_id)
    where preferred and status = 'active';

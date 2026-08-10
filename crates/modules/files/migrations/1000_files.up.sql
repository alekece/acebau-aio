create table file_metadata (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    storage_key text not null unique,
    original_filename text not null,
    media_type text not null,
    size_bytes bigint not null check (size_bytes >= 0),
    owner_kind text not null,
    owner_id uuid not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index file_metadata_owner_idx on file_metadata(owner_kind, owner_id);

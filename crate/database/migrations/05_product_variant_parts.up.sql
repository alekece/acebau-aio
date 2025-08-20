create function check_filament_type()
    returns trigger as
$$
begin
  if (select type from materials where id = new.filament_id) <> 'filament' then
    raise exception 'filament_id must reference a material with type = filament';
  end if;
  return new;
end;
$$ language plpgsql;

create table product_variant_parts
(
    id uuid primary key default gen_random_uuid(),
    product_variant_id uuid references product_variants(id) on delete cascade,
    part_id uuid references parts(id) on delete cascade,
    filament_id uuid references materials(id) on delete cascade,
    quantity integer not null default 1,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

create trigger enforce_filament_type
before insert or update on product_variant_parts
for each row
execute function check_filament_type();

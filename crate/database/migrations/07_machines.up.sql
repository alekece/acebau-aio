create or replace function set_maintenance_cost()
    returns trigger as
$$
begin
  if new.annual_maintenance_cost is null then
    new.annual_maintenance_cost := new.price * 0.1;
  end if;
  return new;
end;
$$ language plpgsql;

create table machine_models
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    name text not null,
    amortized_lifetime interval not null,
    average_power_consumption numeric not null,
    price numeric not null,
    additional_pieces_cost numeric not null default 0,
    annual_maintenance_cost numeric,
    print_width numeric not null,
    print_length numeric not null,
    print_height numeric not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

create trigger default_maintenance_cost
before insert or update on machine_models
for each row execute function set_maintenance_cost();

create table machines
(
    id uuid primary key default gen_random_uuid(),
    status status not null default 'draft',
    machine_model_id uuid not null references machine_models(id) on delete cascade,
    printing_environment_id uuid not null references printing_environments(id) on delete cascade,
    nickname text not null,
    purchase_date date not null,
    total_printing_duration interval,
    last_maintenance_at date,
    cost_buffer_factor numeric not null default 1.3 check (cost_buffer_factor >= 1),
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

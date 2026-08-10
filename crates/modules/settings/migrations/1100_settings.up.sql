create table application_setting (
    id uuid primary key default '00000000-0000-0000-0000-000000000001',
    status status not null default 'active',
    default_time_unit text not null default 'h' check (default_time_unit in ('min', 'h', 'd', 'y')),
    default_mass_unit text not null default 'g' check (default_mass_unit in ('g', 'kg')),
    default_length_unit text not null default 'mm' check (default_length_unit in ('mm', 'cm', 'm')),
    default_power_unit text not null default 'W' check (default_power_unit in ('W', 'kW')),
    default_page_size integer not null default 10 check (default_page_size in (10, 25, 50, 100)),
    electricity_rate text not null default '0.25/kWh',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    check (id = '00000000-0000-0000-0000-000000000001')
);

insert into application_setting default values;

create type expense_payment_state as enum ('unpaid', 'paid');

create table expense (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    activity_id uuid references activity(id),
    supplier text not null,
    accounting_date date not null,
    category text not null,
    description text not null,
    total_ht numeric not null check (total_ht >= 0),
    total_vat numeric not null check (total_vat >= 0),
    total_ttc numeric not null check (total_ttc >= 0 and total_ttc = total_ht + total_vat),
    payment_state expense_payment_state not null default 'paid',
    payment_method text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index expense_activity_id_idx on expense(activity_id);
create index expense_accounting_date_idx on expense(accounting_date);

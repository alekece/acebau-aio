create type invoice_payment_state as enum ('imported', 'unpaid', 'paid', 'credited');

create table imported_invoice (
    id uuid primary key default gen_random_uuid(),
    status status not null default 'active',
    external_platform text not null,
    external_identifier text not null,
    number text not null unique,
    customer text not null,
    activity_id uuid references activity(id),
    order_id uuid references customer_order(id),
    issued_on date not null,
    due_on date not null,
    total_ht numeric not null check (total_ht >= 0),
    total_vat numeric not null check (total_vat >= 0),
    total_ttc numeric not null check (total_ttc >= 0 and total_ttc = total_ht + total_vat),
    payment_state invoice_payment_state not null default 'imported',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (external_platform, external_identifier),
    check (due_on >= issued_on)
);

create index imported_invoice_activity_id_idx on imported_invoice(activity_id);
create index imported_invoice_order_id_idx on imported_invoice(order_id);

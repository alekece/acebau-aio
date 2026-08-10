create type production_job_state as enum ('to_print', 'printing', 'done', 'failed');

alter table production_task_line
    add column machine_id uuid references machine(id),
    add column state production_job_state not null default 'to_print',
    add column started_at timestamptz,
    add column failure_reason text,
    add column failed_quantity integer check (failed_quantity is null or failed_quantity > 0),
    add column actual_waste_grams double precision check (actual_waste_grams is null or actual_waste_grams >= 0);

create index production_task_line_machine_id_idx on production_task_line(machine_id);

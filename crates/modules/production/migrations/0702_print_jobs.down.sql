drop index production_task_line_machine_id_idx;

alter table production_task_line
    drop column actual_waste_grams,
    drop column failed_quantity,
    drop column failure_reason,
    drop column started_at,
    drop column state,
    drop column machine_id;

drop type production_job_state;

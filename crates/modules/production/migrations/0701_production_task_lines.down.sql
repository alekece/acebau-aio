drop table production_task_line;
alter index production_task_legacy_variant_id_idx rename to production_task_variant_id_idx;
alter table production_task rename constraint production_task_legacy_variant_id_fkey to production_task_variant_id_fkey;
alter table production_task rename column legacy_variant_id to variant_id;
alter table production_task rename column legacy_quantity to quantity;
alter table production_task alter column reference drop default;
drop sequence production_reference_seq;

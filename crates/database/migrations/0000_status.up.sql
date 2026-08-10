do $$
begin
    create type status as enum ('active', 'draft', 'archived');
exception
    when duplicate_object then null;
end $$;

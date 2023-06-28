-- Add up migration script here

create table if not exists restaurants (
  id uuid primary key default uuid_generate_v4(),
  name varchar(128) not null,
  address varchar(256) not null,
  email varchar(128) not null,
  phone varchar(11) not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create trigger update_timestamp
before update on restaurants
for each row
execute procedure trigger_set_timestamp();
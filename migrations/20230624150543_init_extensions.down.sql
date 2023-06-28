-- Add down migration script here

drop extension if exists "uuid-owasp";
drop function if exists trigger_set_timestamp();
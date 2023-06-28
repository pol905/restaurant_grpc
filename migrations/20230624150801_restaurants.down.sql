-- Add down migration script here
drop trigger if exists update_timestamp on restaurants;
drop table if exists restaurants;
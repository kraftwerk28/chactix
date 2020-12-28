create table users (
  user_id serial primary key,
  username varchar
);

create table messages (
  message_id serial primary key,
  local_user_id int,
  time timestamp default now(),
  text varchar
);

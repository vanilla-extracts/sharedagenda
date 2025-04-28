create table if not exists agenda.users(
        uuid varchar(255) primary key,
        email varchar(50) not null,
        name varchar(50) not null,
        password varchar(50) not null
);

create table if not exists agenda.events(
	id serial primary key,
        owner varchar(255) references agenda.users(uuid) not null,
        name varchar(255) not null,
        date_start timestamp with time zone not null,
        date_end timestamp with time zone not null
);


create table if not exists agenda.token(
	id serial primary key,
        token varchar(255) not null,
        owner varchar(255) not null references agenda.users(uuid),
        expiration_date timestamp with time zone not null
);


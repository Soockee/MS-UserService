CREATE TABLE meta_info (
    db_version integer NOT NULL
);

CREATE TABLE user (
    guid serial,
    username varchar(40),
    email varchar(100) CONSTRAINT UNIQUE
);
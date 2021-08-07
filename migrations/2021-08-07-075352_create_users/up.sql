CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    guid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL
)
-- CREATE EXTENSION citext;
-- CREATE DOMAIN email AS citext
--  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    created timestamptz NOT NULL,
    last_updated timestamptz NOT NULL,
    pw_hash TEXT NOT NULL
);
CREATE TABLE transactions (
    serial SERIAL PRIMARY KEY,
    sender UUID NOT NULL,
    recipient UUID,
    trans_type SmallInt NOT NULL,
    amount BigInt NOT NULL,
    currency Text NOT NULL,
    date timestamptz NOT NULL
);
CREATE TABLE owners (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    registered timestamptz,
    email email,
    phone_number TEXT,
    date_of_birth timestamptz
);

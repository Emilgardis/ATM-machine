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
)

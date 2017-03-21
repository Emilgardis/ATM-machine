CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    created timestamptz NOT NULL,
    last_updated timestamptz NOT NULL,
    pw_hash TEXT NOT NULL
)

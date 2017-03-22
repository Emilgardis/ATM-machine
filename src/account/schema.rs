table! {
    use diesel::types::*;
    accounts (id) {
        id -> Uuid,
        owner_id -> Uuid,
        created -> Timestamptz,
        last_updated -> Timestamptz,
        pw_hash -> VarChar,
    }
}

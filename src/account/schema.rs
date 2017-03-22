table! {
    use diesel::types::*;
    use account::Owner;
    accounts (id) {
        id -> Uuid,
        owner_id -> Uuid,
        created -> Timestamptz,
        last_updated -> Timestamptz,
        pw_hash -> VarChar,
    }
}

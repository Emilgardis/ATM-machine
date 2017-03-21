table! {
    use diesel::types::*;
    use account::Owner;
    accounts (id) {
        id -> Uuid,
        last_updated -> Timestamptz,
        created -> Timestamptz,
        pw_hash -> VarChar,
        owner_id -> Uuid,

    }
}

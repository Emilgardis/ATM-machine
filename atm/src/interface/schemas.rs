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

table! {
    use diesel::types::*;
    transactions (serial) {
        serial -> Serial,
        sender -> Uuid,
        recipient -> Nullable<Uuid>,
        trans_type -> SmallInt,
        amount -> BigInt,
        currency -> Text,
        date -> Timestamptz,
    }
}

table! {
    use diesel::types::*;
    owners (id) {
        id -> Uuid,
        name -> Text,
        registered -> Timestamptz,
        email -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        date_of_birth -> Nullable<Timestamptz>,
    }
}

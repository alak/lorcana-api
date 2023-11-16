// @generated automatically by Diesel CLI.

diesel::table! {
    price_records (created_at) {
        created_at -> Timestamp,
        card_id -> Varchar,
        set_code -> Varchar,
        number -> Int4,
        min_price -> Float8,
        avg_price -> Float8,
        is_foil -> Bool,
        locale -> Varchar,
    }
}

diesel::table! {
    prices (id) {
        id -> Varchar,
        created_at -> Timestamp,
        card_id -> Varchar,
        price -> Float8,
        seller_location -> Varchar,
        locale -> Varchar,
    }
}

diesel::joinable!(prices -> price_records (created_at));

diesel::allow_tables_to_appear_in_same_query!(
    price_records,
    prices,
);

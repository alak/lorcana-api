// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Varchar,
        created_at -> Timestamp,
        set_code -> Varchar,
        number -> Int4,
        set -> Varchar,
        name -> Varchar,
        color -> Varchar,
        rarity -> Varchar,
        card_market_handle -> Varchar,
    }
}

diesel::table! {
    localized_colors (locale, color) {
        color -> Varchar,
        localized_color -> Varchar,
        locale -> Varchar,
    }
}

diesel::table! {
    localized_names (id) {
        id -> Varchar,
        card_id -> Varchar,
        name -> Varchar,
        locale -> Varchar,
    }
}

diesel::table! {
    localized_rarities (locale, rarity) {
        rarity -> Varchar,
        localized_rarity -> Varchar,
        locale -> Varchar,
    }
}

diesel::table! {
    prices (created_at) {
        created_at -> Timestamp,
        min_price -> Float8,
        avg_price -> Float8,
        card_id -> Varchar,
        is_foil -> Bool,
        locale -> Varchar,
    }
}

diesel::joinable!(localized_names -> cards (card_id));
diesel::joinable!(prices -> cards (card_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    localized_colors,
    localized_names,
    localized_rarities,
    prices,
);

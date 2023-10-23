use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::{cards, localized_colors, localized_names, localized_rarities, prices};

#[derive(
    Queryable, Selectable, Insertable, Identifiable, Associations, PartialEq, Debug, Clone,
)]
#[diesel(belongs_to(Card))]
#[diesel(table_name = localized_names)]
pub struct LocalizedName {
    pub id: String, // fr-TFC-1
    pub card_id: String,
    pub name: String,   // Ariel, sur ses jambes
    pub locale: String, // fr
}

impl LocalizedName {
    pub fn new(_id: String, _card_id: String, _name: String, _locale: String) -> Self {
        Self {
            id: _id,
            card_id: _card_id,
            name: _name,
            locale: _locale,
        }
    }
}

#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = localized_colors)]
pub struct LocalizedColor {
    pub color: String,           // amber
    pub localized_color: String, // ambre
    pub locale: String,          // fr
}

#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = localized_rarities)]
pub struct LocalizedRarity {
    pub rarity: String,           // fr-common
    pub localized_rarity: String, // commun
    pub locale: String,           // fr
}

#[derive(
    Queryable, Selectable, Identifiable, Insertable, Associations, PartialEq, Debug, Clone,
)]
#[diesel(belongs_to(Card))]
#[diesel(primary_key(created_at))]
#[diesel(table_name = prices)]
pub struct Price {
    pub created_at: NaiveDateTime,
    pub card_id: String, // TFC-1
    pub min_price: f64,  // 0.5
    pub avg_price: f64,  // 0.6
    pub is_foil: bool,   // false
    pub locale: String,  // en
}

impl Price {
    pub fn new(
        _card_id: String,
        _created_at: NaiveDateTime,
        _min_price: f64,
        _avg_price: f64,
        _is_foil: bool,
        _locale: String,
    ) -> Self {
        Self {
            created_at: _created_at,
            card_id: _card_id,
            min_price: _min_price,
            avg_price: _avg_price,
            is_foil: _is_foil,
            locale: _locale,
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub set_code: String,
    pub number: i32,
    pub set: String,
    pub name: String,
    pub color: String,
    pub rarity: String,
    pub card_market_handle: String,
}

impl Card {
    pub fn new(
        _id: String,
        _created_at: NaiveDateTime,
        _set_code: String,
        _number: i32,
        _set: String,
        _name: String,
        _color: String,
        _rarity: String,
        _card_market_handle: String,
    ) -> Self {
        Self {
            id: _id,
            created_at: _created_at,
            set_code: _set_code,
            number: _number,
            set: _set,
            name: _name,
            color: _color,
            rarity: _rarity,
            card_market_handle: _card_market_handle,
        }
    }
}

use crate::model::*;
use chrono::{DateTime, TimeZone, Utc};
extern crate serde_derive;
use serde::Serialize;
use std::collections::HashMap;
use serde_with::TimestampMilliSeconds;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ResponsePricedCard {
    pub id: String,                      // TFC-1
    pub set_code: String,                // TFC
    pub number: i32,                     // 1
    pub set: String,                     // The First Chapter
    pub name: String,                    // Ariel, on her legs
    pub color: String,                   // amber
    pub rarity: String,                  // common
    pub card_market_handle: String,      // Ariel-On-Her-Legs
    pub locale: String,                  // en
    pub min_price: f64,                  // 0.5
    pub avg_price: f64,                  // 0.6
    pub is_foil: bool,                   // false
    #[serde_as(as = "TimestampMilliSeconds<i64>")]             
    pub price_created_at: DateTime<Utc>, // TimestampMilliSeconds
}

impl ResponsePricedCard {
    pub fn new(
        card: Card,
        price: Price,
        localized_name: LocalizedName,
        localized_color: LocalizedColor,
        localized_rarity: LocalizedRarity,
    ) -> ResponsePricedCard {
        ResponsePricedCard {
            id: card.id,
            set_code: card.set_code,
            number: card.number,
            set: card.set,
            name: localized_name.name.to_string(),
            color: localized_color.localized_color.to_string(),
            rarity: localized_rarity.localized_rarity.to_string(),
            card_market_handle: card.card_market_handle,
            locale: price.locale,
            min_price: price.min_price,
            avg_price: price.avg_price,
            is_foil: price.is_foil,
            price_created_at: Utc.from_utc_datetime(&price.created_at),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseLocalizedCard {
    pub id: String,                 // TFC-1
    pub set_code: String,           // TFC
    pub number: i32,                // 1
    pub set: String,                // The First Chapter
    pub name: String,               // Ariel, on her legs
    pub color: String,              // amber
    pub rarity: String,             // common
    pub card_market_handle: String, // Ariel-On-Her-Legs
    pub locale: String,             // en
}

#[allow(dead_code)]
impl ResponseLocalizedCard {
    pub fn new(
        _locale: String,
        card: Card,
        localized_names: HashMap<String, String>,
        localized_colors: HashMap<String, String>,
        localized_rarities: HashMap<String, String>,
    ) -> Self {
        Self {
            id: card.id,
            set_code: card.set_code,
            number: card.number,
            set: card.set,
            name: localized_names.get(&_locale).unwrap().to_string(),
            color: localized_colors.get(&_locale).unwrap().to_string(),
            rarity: localized_rarities.get(&_locale).unwrap().to_string(),
            card_market_handle: card.card_market_handle,
            locale: _locale,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseFullCard {
    pub id: String,                        // TFC-1
    pub set_code: String,                  // TFC
    pub number: i32,                       // 1
    pub set: String,                       // The First Chapter
    pub names: HashMap<String, String>, // {"en": "Ariel, on her legs", "fr": "Ariel, sur ses jambes"}
    pub colors: HashMap<String, String>, // {"en": "amber", "fr": "ambre"}
    pub rarities: HashMap<String, String>, // {"en": "common", "fr": "commune"}
    pub card_market_handle: String,     // Ariel-On-Her-Legs
}

#[allow(dead_code)]
impl ResponseFullCard {
    pub fn new(
        card: Card,
        localized_names: HashMap<String, String>,
        localized_colors: HashMap<String, String>,
        localized_rarities: HashMap<String, String>,
    ) -> Self {
        Self {
            id: card.id,
            set_code: card.set_code,
            number: card.number,
            set: card.set,
            names: localized_names,
            colors: localized_colors,
            rarities: localized_rarities,
            card_market_handle: card.card_market_handle,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ResponseMultiPrice {
    pub id: String,                           // TFC-1
    pub locale: String,                       // en
    pub min_price: f64,                       // 0.5
    pub avg_price: f64,                       // 0.6
    #[serde_as(as = "TimestampMilliSeconds<i64>")]  
    pub price_created_at: DateTime<Utc>,      // 2021-01-01T00:00:00Z
    pub foil_min_price: f64,                  // 1.0
    pub foil_avg_price: f64,                  // 2.0
    #[serde_as(as = "TimestampMilliSeconds<i64>")] 
    pub foil_price_created_at: DateTime<Utc>, // 2021-01-01T00:00:00Z
}

impl ResponseMultiPrice {
    pub fn new(price: Price, foil_price: Price) -> ResponseMultiPrice {
        ResponseMultiPrice {
            id: price.card_id,
            locale: price.locale,
            min_price: price.min_price,
            avg_price: price.avg_price,
            price_created_at: Utc.from_utc_datetime(&price.created_at),
            foil_min_price: foil_price.min_price,
            foil_avg_price: foil_price.avg_price,
            foil_price_created_at: Utc.from_utc_datetime(&foil_price.created_at),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponsePricesWrapper {
    pub foil: Vec<ResponsePrice>,                           
    pub standard: Vec<ResponsePrice>,                      
}

impl ResponsePricesWrapper {
    pub fn new(_foil: Vec<Price>, _standard: Vec<Price>) -> ResponsePricesWrapper {
        let f =  _foil.into_iter().map(|price| { ResponsePrice::new(price, true) }).collect();
        let std = _standard.into_iter().map(|price| { ResponsePrice::new(price, false) }).collect();
        ResponsePricesWrapper {
            foil: f,
            standard: std,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ResponsePrice {
    pub id: String,                           // TFC-1
    pub locale: String,                       // en
    pub min_price: f64,                       // 0.5
    pub avg_price: f64,                       // 0.6
    pub is_foil: bool,                        // false
    #[serde_as(as = "TimestampMilliSeconds<i64>")]  
    pub created_at: DateTime<Utc>,      // 2021-01-01T00:00:00Z
}

impl ResponsePrice {
    pub fn new(price: Price, _is_foil: bool) -> ResponsePrice {
        ResponsePrice {
            id: price.card_id,
            locale: price.locale,
            min_price: price.min_price,
            avg_price: price.avg_price,
            is_foil: _is_foil,
            created_at: Utc.from_utc_datetime(&price.created_at),
        }
    }
}

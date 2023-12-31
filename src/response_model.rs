use crate::model::*;
use chrono::{DateTime, TimeZone, Utc};
extern crate serde_derive;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::TimestampMilliSeconds;
use std::collections::HashMap;

// #[serde_as]
// #[derive(Debug, Serialize)]
// pub struct ResponsePricedCard {
//     pub id: String,                      // TFC-1
//     pub set_code: String,                // TFC
//     pub number: i32,                     // 1
//     pub set: String,                     // The First Chapter
//     pub name: String,                    // Ariel, on her legs
//     pub color: String,                   // amber
//     pub rarity: String,                  // common
//     pub card_market_handle: String,      // Ariel-On-Her-Legs
//     pub locale: String,                  // en
//     pub min_price: f64,                  // 0.5
//     pub avg_price: f64,                  // 0.6
//     pub is_foil: bool,                   // false
//     #[serde_as(as = "TimestampMilliSeconds<i64>")]
//     pub price_created_at: DateTime<Utc>, // TimestampMilliSeconds
// }

// impl ResponsePricedCard {
//     pub fn new(
//         card: Card,
//         price: Price,
//         localized_name: LocalizedName,
//         localized_color: LocalizedColor,
//         localized_rarity: LocalizedRarity,
//     ) -> ResponsePricedCard {
//         ResponsePricedCard {
//             id: card.id,
//             set_code: card.set_code,
//             number: card.number,
//             set: card.set,
//             name: localized_name.name.to_string(),
//             color: localized_color.localized_color.to_string(),
//             rarity: localized_rarity.localized_rarity.to_string(),
//             card_market_handle: card.card_market_handle,
//             locale: price.locale,
//             min_price: price.min_price,
//             avg_price: price.avg_price,
//             is_foil: price.is_foil,
//             price_created_at: Utc.from_utc_datetime(&price.created_at),
//         }
//     }
// }

// #[derive(Debug, Serialize)]
// pub struct ResponseLocalizedCard {
//     pub id: String,                 // TFC-1
//     pub set_code: String,           // TFC
//     pub number: i32,                // 1
//     pub set: String,                // The First Chapter
//     pub name: String,               // Ariel, on her legs
//     pub color: String,              // amber
//     pub rarity: String,             // common
//     pub card_market_handle: String, // Ariel-On-Her-Legs
//     pub locale: String,             // en
// }

// #[allow(dead_code)]
// impl ResponseLocalizedCard {
//     pub fn new(
//         _locale: String,
//         card: Card,
//         localized_names: HashMap<String, String>,
//         localized_colors: HashMap<String, String>,
//         localized_rarities: HashMap<String, String>,
//     ) -> Self {
//         Self {
//             id: card.id,
//             set_code: card.set_code,
//             number: card.number,
//             set: card.set,
//             name: localized_names.get(&_locale).unwrap().to_string(),
//             color: localized_colors.get(&_locale).unwrap().to_string(),
//             rarity: localized_rarities.get(&_locale).unwrap().to_string(),
//             card_market_handle: card.card_market_handle,
//             locale: _locale,
//         }
//     }
// }

// #[derive(Debug, Serialize)]
// pub struct ResponseFullCard {
//     pub id: String,                        // TFC-1
//     pub set_code: String,                  // TFC
//     pub number: i32,                       // 1
//     pub set: String,                       // The First Chapter
//     pub names: HashMap<String, String>, // {"en": "Ariel, on her legs", "fr": "Ariel, sur ses jambes"}
//     pub colors: HashMap<String, String>, // {"en": "amber", "fr": "ambre"}
//     pub rarities: HashMap<String, String>, // {"en": "common", "fr": "commune"}
//     pub card_market_handle: String,     // Ariel-On-Her-Legs
// }

// #[allow(dead_code)]
// impl ResponseFullCard {
//     pub fn new(
//         card: Card,
//         localized_names: HashMap<String, String>,
//         localized_colors: HashMap<String, String>,
//         localized_rarities: HashMap<String, String>,
//     ) -> Self {
//         Self {
//             id: card.id,
//             set_code: card.set_code,
//             number: card.number,
//             set: card.set,
//             names: localized_names,
//             colors: localized_colors,
//             rarities: localized_rarities,
//             card_market_handle: card.card_market_handle,
//         }
//     }
// }

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ResponseMultiPriceRecord {
    pub id: String,     // TFC-1
    pub locale: String, // en
    pub min_price: f64, // 0.5
    pub avg_price: f64, // 0.6
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub price_created_at: DateTime<Utc>, // 2021-01-01T00:00:00Z
    pub foil_min_price: f64, // 1.0
    pub foil_avg_price: f64, // 2.0
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub foil_price_created_at: DateTime<Utc>, // 2021-01-01T00:00:00Z
}

impl ResponseMultiPriceRecord {
    pub fn new(price: PriceRecord, foil_price: PriceRecord) -> ResponseMultiPriceRecord {
        ResponseMultiPriceRecord {
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
    pub foil: Vec<ResponsePriceRecord>,
    pub standard: Vec<ResponsePriceRecord>,
}

impl ResponsePricesWrapper {
    pub fn new(
        _foil: Vec<ResponsePriceRecord>,
        _standard: Vec<ResponsePriceRecord>,
    ) -> ResponsePricesWrapper {
        ResponsePricesWrapper {
            foil: _foil,
            standard: _standard,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub struct ResponsePriceRecord {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: DateTime<Utc>, // 2021-01-01T00:00:00Z
    pub card_id: String,
    pub number: i32,
    pub set_code: String, // TFC-1
    pub locale: String,   // en
    pub min_price: f64,
    pub avg_price: f64, // 0.5                     // 0.6                     // false
    pub is_foil: bool,  // true
    pub prices: Vec<ResponsePrice>,
}

impl ResponsePriceRecord {
    pub fn new(record: PriceRecord, _prices: Vec<ResponsePrice>) -> ResponsePriceRecord {
        ResponsePriceRecord {
            created_at: Utc.from_utc_datetime(&record.created_at),
            card_id: record.card_id,
            number: record.number,
            set_code: record.set_code,
            locale: record.locale,
            min_price: record.min_price,
            avg_price: record.avg_price,
            is_foil: record.is_foil,
            prices: _prices,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub struct ResponsePrice {
    pub id: String,
    pub card_id: String,         // TFC-1
    pub locale: String,          // en
    pub price: f64,              // 0.5
    pub seller_location: String, // 0.6                     // false
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: DateTime<Utc>, // 2021-01-01T00:00:00Z
}

impl ResponsePrice {
    pub fn new(price: Price) -> ResponsePrice {
        ResponsePrice {
            id: price.id,
            card_id: price.card_id,
            locale: price.locale,
            price: price.price,
            seller_location: price.seller_location,
            created_at: Utc.from_utc_datetime(&price.created_at),
        }
    }
}

use crate::model::*;
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InsertRequestCard {
    pub set_code: String,           // TFC
    pub number: i32,                // 1
    pub set: String,                // The First Chapter
    pub name: String,               // Ariel, on her legs
    pub fr_name: String,            // Ariel, sur ses jambes
    pub color: String,              // amber
    pub rarity: String,             // common
    pub card_market_handle: String, // Ariel-On-Her-Legs
}

impl InsertRequestCard {
    pub fn to_card(&self) -> Card {
        Card::new(
            format!("{}-{}", self.set_code, self.number),
            Utc::now().naive_utc(),
            self.set_code.clone(),
            self.number.clone(),
            self.set.clone(),
            self.name.clone(),
            self.color.clone(),
            self.rarity.clone(),
            self.card_market_handle.clone(),
        )
    }

    pub fn to_names(&self) -> Vec<LocalizedName> {
        vec![
            LocalizedName::new(
                format!("en-{}-{}", self.set_code, self.number),
                format!("{}-{}", self.set_code, self.number),
                self.name.clone(),
                "en".to_string(),
            ),
            LocalizedName::new(
                format!("fr-{}-{}", self.set_code, self.number),
                format!("{}-{}", self.set_code, self.number),
                self.fr_name.clone(),
                "fr".to_string(),
            ),
        ]
    }
}

#[derive(Debug, Deserialize)]
pub struct InsertRequestPrice {
    pub set_code: String,
    pub number: i32,
    pub min_price: f64, // 0.5
    pub avg_price: f64, // 0.6
    pub is_foil: bool,  // false
    pub locale: String, // en
}

impl InsertRequestPrice {
    pub fn to_price(&self) -> Price {
        Price::new(
            format!("{}-{}", self.set_code, self.number),
            Utc::now().naive_utc(),
            self.min_price.clone(),
            self.avg_price.clone(),
            self.is_foil.clone(),
            self.locale.clone(),
        )
    }
}

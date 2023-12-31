use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::result::Error as DieselError;

use crate::bearer_auth::BearerToken;
use crate::model::*;
use crate::request_insert_model::*;
use crate::response_model::*;
use crate::schema::*;

use chrono::Utc;
use diesel::prelude::*;
use qstring::QString;

use crate::constants::CONNECTION_POOL_ERROR;

use crate::{DBPool, DBPooledConnection};

use std::fmt;

#[derive(serde::Serialize)]
struct SerializableError {
    message: String,
}

impl From<DieselError> for SerializableError {
    fn from(error: DieselError) -> Self {
        SerializableError {
            message: error.to_string(),
        }
    }
}

impl fmt::Debug for SerializableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("message", &self.message)
            .finish()
    }
}

/// create a price `/prices`
#[post("/prices")]
pub async fn create(
    price_req: Json<InsertRequestPriceRecord>,
    pool: Data<DBPool>,
    _token: BearerToken,
) -> actix_web::Result<impl Responder> {
    println!("create prices");
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let date = Utc::now().naive_utc();

    let mut _prices = price_req
        .prices
        .iter()
        .map(|price| price.to_price(date.clone()))
        .collect::<Vec<Price>>();

    let price = web::block(move || {
        create_price_record(
            price_req.to_price_record(date.clone()),
            _prices.clone(),
            &mut conn,
        )
    })
    .await;

    Ok(match price {
        Ok(_) => HttpResponse::Created().finish(),
        _ => HttpResponse::NoContent().await.unwrap(),
    })
}

fn create_price_record(
    _price_record: PriceRecord,
    _prices: Vec<Price>,
    conn: &mut DBPooledConnection,
) -> Result<PriceRecord, DieselError> {
    use crate::schema::price_records::dsl::*;
    use crate::schema::prices::dsl::*;

    let _ = diesel::insert_into(price_records)
        .values(&_price_record)
        .execute(conn)
        .expect("Can't create Price Record");

    for _price in _prices.iter() {
        let _ = diesel::insert_into(prices)
            .values(_price)
            .execute(conn)
            .expect("Can't create Price");
    }

    Ok(_price_record)
}

// ///get last price of a given price `/prices/{set_code}/{number}`
// #[get("/prices/{set_code}/{number}")]
// pub async fn get(
//     path: Path<(String, String)>,
//     pool: Data<DBPool>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     println!("get prices");
//     let query_str = req.query_string();
//     let qs = QString::from(query_str);
//     let is_foil_string = qs.get("is_foil").unwrap_or("false");
//     let is_foil = value_to_bool(&is_foil_string.to_string());
//     let locale = qs.get("locale").unwrap_or("en").to_string();

//     let (set_code, number) = path.into_inner();

//     let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
//     let price_response = web::block(move || {
//         find_last_price(
//             format!("{}-{}", set_code, number),
//             is_foil,
//             locale,
//             &mut conn,
//         )
//     })
//     .await;

//     match price_response {
//         Ok(price) => HttpResponse::Ok()
//             .json(price.unwrap()),
//         _ => HttpResponse::NotFound().await.unwrap(),
//     }
// }

// fn find_last_price(
//     _card_id: String,
//     _is_foil: bool,
//     _locale: String,
//     conn: &mut DBPooledConnection,
// ) -> Result<ResponsePricedCard, SerializableError> {
//     let _card = cards::table
//         .filter(cards::id.eq(_card_id.clone()))
//         .select(Card::as_select())
//         .get_result(conn)
//         .expect("Card not found");

//     let _localized_name = LocalizedName::belonging_to(&_card)
//         .filter(localized_names::locale.eq(_locale.clone()))
//         .select(LocalizedName::as_select())
//         .first(conn)
//         .expect("Localized name not found");

//     println!(
//         "color: {}, locale: {}",
//         _card.color.clone(),
//         _locale.clone()
//     );

//     let _localized_color = localized_colors::table
//         .filter(localized_colors::color.eq(_card.color.to_lowercase().clone()))
//         .filter(localized_colors::locale.eq(_locale.clone()))
//         .select(LocalizedColor::as_select())
//         .first(conn)
//         .expect("Localized color not found");

//     let _localized_rarity = localized_rarities::table
//         .filter(localized_rarities::rarity.eq(_card.rarity.to_lowercase().clone()))
//         .filter(localized_rarities::locale.eq(_locale.clone()))
//         .select(LocalizedRarity::as_select())
//         .first(conn)
//         .expect("Localized rarity not found");

//     let _price = Price::belonging_to(&_card)
//         .select(Price::as_select())
//         .filter(prices::is_foil.eq(_is_foil.clone()))
//         .filter(prices::locale.eq(_locale.clone()))
//         .order(prices::created_at.desc())
//         .first(conn)
//         .expect("Price not found");

//     return Ok(ResponsePricedCard::new(
//         _card,
//         _price,
//         _localized_name,
//         _localized_color,
//         _localized_rarity,
//     ));
// }

///get all prices `/all_prices/{set_code}`
#[get("/all_prices/{set_code}")]
pub async fn get_all_prices(
    path: Path<String>,
    pool: Data<DBPool>,
    req: HttpRequest,
) -> HttpResponse {
    println!("get all prices");
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let locale = qs.get("locale").unwrap_or("en").to_string();

    let set_code = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let prices_response = web::block(move || find_all_prices(set_code, locale, &mut conn)).await;

    match prices_response {
        Ok(resp) => HttpResponse::Ok().json(resp.unwrap()),
        _ => HttpResponse::NotFound().await.unwrap(),
    }
}

fn find_all_prices(
    _set_code: String,
    _locale: String,
    conn: &mut DBPooledConnection,
) -> Result<Vec<ResponseMultiPriceRecord>, SerializableError> {
    let _prices = price_records::table
        .order((
            price_records::number.asc(),
            price_records::created_at.desc(),
        ))
        .distinct_on(price_records::number)
        .filter(price_records::locale.eq(_locale.clone()))
        .filter(price_records::is_foil.eq(false))
        .filter(price_records::set_code.eq(_set_code.clone()))
        .select(PriceRecord::as_select())
        .load(conn)
        .unwrap();

    let _foil_prices = price_records::table
        .order((
            price_records::number.asc(),
            price_records::created_at.desc(),
        ))
        .distinct_on(price_records::number)
        .filter(price_records::locale.eq(_locale.clone()))
        .filter(price_records::is_foil.eq(true))
        .filter(price_records::set_code.eq(_set_code.clone()))
        .select(PriceRecord::as_select())
        .load(conn)
        .unwrap();

    if _prices.len() != _foil_prices.len() {
        return Err(DieselError::NotFound.into());
    }

    let mut _multi_prices = Vec::new();

    for i in 0.._prices.len() {
        let _price = _prices[i].clone();
        let _foil_price = _foil_prices[i].clone();

        let resp = ResponseMultiPriceRecord::new(_price, _foil_price);

        _multi_prices.push(resp);
    }

    return Ok(_multi_prices);
}

// /get all prices `/all_grouped_prices/{set_code}`
#[get("/all_grouped_prices/{set_code}")]
pub async fn all_grouped_prices(
    path: Path<String>,
    pool: Data<DBPool>,
    req: HttpRequest,
) -> HttpResponse {
    println!("get all prices");
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let locale = qs.get("locale").unwrap_or("en").to_string();

    let set_code = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let prices_response =
        web::block(move || find_all_grouped_prices(set_code, locale, &mut conn)).await;

    match prices_response {
        Ok(resp) => HttpResponse::Ok().json(resp.unwrap()),
        _ => HttpResponse::NotFound().await.unwrap(),
    }
}

fn find_all_grouped_prices(
    _set_code: String,
    _locale: String,
    conn: &mut DBPooledConnection,
) -> Result<ResponsePricesWrapper, SerializableError> {
    let _price_records = price_records::table
        .order((
            price_records::number.asc(),
            price_records::created_at.desc(),
        ))
        .distinct_on(price_records::number)
        .filter(price_records::locale.eq(_locale.clone()))
        .filter(price_records::is_foil.eq(false))
        .filter(price_records::set_code.eq(_set_code.clone()))
        .select(PriceRecord::as_select())
        .load(conn)
        .unwrap();

    let _prices = Price::belonging_to(&_price_records)
        .select(Price::as_select())
        .load(conn)
        .expect("Price name not found");

    let zip_std = _prices
        .grouped_by(&_price_records)
        .into_iter()
        .zip(_price_records.clone())
        .map(|(_prices, _price_record)| (_price_record, _prices))
        .collect::<Vec<(PriceRecord, Vec<Price>)>>();

    let _foil_price_records = price_records::table
        .order((
            price_records::number.asc(),
            price_records::created_at.desc(),
        ))
        .distinct_on(price_records::number)
        .filter(price_records::locale.eq(_locale.clone()))
        .filter(price_records::is_foil.eq(true))
        .filter(price_records::set_code.eq(_set_code.clone()))
        .select(PriceRecord::as_select())
        .load(conn)
        .unwrap();

    let _foil_prices = Price::belonging_to(&_foil_price_records)
        .select(Price::as_select())
        .load(conn)
        .expect("Price name not found");

    let zip_foil = _foil_prices
        .grouped_by(&_foil_price_records)
        .into_iter()
        .zip(_foil_price_records.clone())
        .map(|(_foil_prices, _foil_price_record)| (_foil_price_record, _foil_prices))
        .collect::<Vec<(PriceRecord, Vec<Price>)>>();

    let mut standart_prices = Vec::new();

    for std in zip_std.iter() {
        let _price_record = std.0.clone();
        let _prices = std.1.clone();

        let resp_prices = _prices
            .iter()
            .map(|p| ResponsePrice::new(p.clone()))
            .collect::<Vec<ResponsePrice>>();

        let standard = ResponsePriceRecord::new(_price_record.clone(), resp_prices.clone());

        standart_prices.push(standard);
    }

    let mut foil_prices = Vec::new();

    for fl in zip_foil.iter() {
        let _price_record = fl.0.clone();
        let _prices = fl.1.clone();

        let resp_prices = _prices
            .iter()
            .map(|p| ResponsePrice::new(p.clone()))
            .collect::<Vec<ResponsePrice>>();

        let foil = ResponsePriceRecord::new(_price_record.clone(), resp_prices.clone());

        foil_prices.push(foil);
    }

    let resp = ResponsePricesWrapper::new(foil_prices.clone(), standart_prices.clone());

    return Ok(resp);
}

fn value_to_bool(value: &str) -> bool {
    let truth_value: bool = match value {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        "1" => true,
        "0" => false,
        _ => false,
    };
    return truth_value;
}

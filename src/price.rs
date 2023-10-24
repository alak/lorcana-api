use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::result::Error;

use crate::bearer_auth::BearerToken;
use crate::model::*;
use crate::request_insert_model::*;
use crate::response_model::*;
use crate::schema::*;

use diesel::prelude::*;

use qstring::QString;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

use crate::{DBPool, DBPooledConnection};

/// create a price `/prices`
#[post("/prices")]
pub async fn create(
    price_req: Json<InsertRequestPrice>,
    pool: Data<DBPool>,
    _token: BearerToken,
) -> HttpResponse {
    println!("create prices");
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let price = web::block(move || create_price(price_req.to_price(), &mut conn)).await;

    match price {
        Ok(_) => HttpResponse::Created().finish(),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

fn create_price(_price: Price, conn: &mut DBPooledConnection) -> Result<Price, Error> {
    use crate::schema::prices::dsl::*;

    let _ = diesel::insert_into(prices)
        .values(&_price)
        .execute(conn)
        .expect("Can't create Price");

    Ok(_price)
}

///get last price of a given price `/prices/{set_code}/{number}`
#[get("/prices/{set_code}/{number}")]
pub async fn get(
    path: Path<(String, String)>,
    pool: Data<DBPool>,
    req: HttpRequest,
) -> HttpResponse {
    println!("get prices");
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let is_foil_string = qs.get("is_foil").unwrap_or("false");
    let is_foil = value_to_bool(&is_foil_string.to_string());
    let locale = qs.get("locale").unwrap_or("en").to_string();

    let (set_code, number) = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let price_response = web::block(move || {
        find_last_price(
            format!("{}-{}", set_code, number),
            is_foil,
            locale,
            &mut conn,
        )
    })
    .await;

    match price_response {
        Ok(price_response) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(price_response),
        _ => HttpResponse::NotFound().await.unwrap(),
    }
}

fn find_last_price(
    _card_id: String,
    _is_foil: bool,
    _locale: String,
    conn: &mut DBPooledConnection,
) -> Result<ResponsePricedCard, Error> {
    let _card = cards::table
        .filter(cards::id.eq(_card_id.clone()))
        .select(Card::as_select())
        .get_result(conn)
        .expect("Card not found");

    let _localized_name = LocalizedName::belonging_to(&_card)
        .filter(localized_names::locale.eq(_locale.clone()))
        .select(LocalizedName::as_select())
        .first(conn)
        .expect("Localized name not found");

    println!(
        "color: {}, locale: {}",
        _card.color.clone(),
        _locale.clone()
    );

    let _localized_color = localized_colors::table
        .filter(localized_colors::color.eq(_card.color.to_lowercase().clone()))
        .filter(localized_colors::locale.eq(_locale.clone()))
        .select(LocalizedColor::as_select())
        .first(conn)
        .expect("Localized color not found");

    let _localized_rarity = localized_rarities::table
        .filter(localized_rarities::rarity.eq(_card.rarity.to_lowercase().clone()))
        .filter(localized_rarities::locale.eq(_locale.clone()))
        .select(LocalizedRarity::as_select())
        .first(conn)
        .expect("Localized rarity not found");

    let _price = Price::belonging_to(&_card)
        .select(Price::as_select())
        .filter(prices::is_foil.eq(_is_foil.clone()))
        .filter(prices::locale.eq(_locale.clone()))
        .order(prices::created_at.desc())
        .first(conn)
        .expect("Price not found");

    return Ok(ResponsePricedCard::new(
        _card,
        _price,
        _localized_name,
        _localized_color,
        _localized_rarity,
    ));
}

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
        Ok(prices_response) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(prices_response),
        _ => HttpResponse::NotFound().await.unwrap(),
    }
}

fn find_all_prices(
    _set_code: String,
    _locale: String,
    conn: &mut DBPooledConnection,
) -> Result<Vec<ResponseMultiPrice>, Error> {
    let _prices = prices::table
        .order((prices::number.asc(), prices::created_at.desc()))
        .distinct_on(prices::number)
        .filter(prices::locale.eq(_locale.clone()))
        .filter(prices::is_foil.eq(false))
        .filter(prices::set_code.eq(_set_code.clone()))
        .select(Price::as_select())
        .load(conn)
        .unwrap();

    let _foil_prices = prices::table
        .order((prices::number.asc(), prices::created_at.desc()))
        .distinct_on(prices::number)
        .filter(prices::locale.eq(_locale.clone()))
        .filter(prices::is_foil.eq(true))
        .filter(prices::set_code.eq(_set_code.clone()))
        .select(Price::as_select())
        .load(conn)
        .unwrap();

    if _prices.len() != _foil_prices.len() {
        return Err(Error::NotFound);
    }

    let mut _multi_prices = Vec::new();

    for i in 0.._prices.len() {
        let _price = _prices[i].clone();
        let _foil_price = _foil_prices[i].clone();

        let resp = ResponseMultiPrice::new(_price, _foil_price);

        _multi_prices.push(resp);
    }

    return Ok(_multi_prices);
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

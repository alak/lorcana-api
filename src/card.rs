use actix_web::{web, web::Data, web::Json, HttpResponse};
use diesel::result::Error;

use crate::constants::CONNECTION_POOL_ERROR;
use crate::{DBPool, DBPooledConnection};

use crate::bearer_auth::BearerToken;
use crate::model::*;
use crate::request_insert_model::*;
use diesel::upsert::on_constraint;
use diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

/// create a card `/cards`
#[post("/cards")]
pub async fn create(
    card_req: Json<InsertRequestCard>,
    pool: Data<DBPool>,
    _token: BearerToken,
) -> HttpResponse {
    println!("create card");
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let card =
        web::block(move || create_card(card_req.to_card(), card_req.to_names(), &mut conn)).await;

    match card {
        Ok(_) => HttpResponse::Created().finish(),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

fn create_card(
    _card: Card,
    _localized_names: Vec<LocalizedName>,
    conn: &mut DBPooledConnection,
) -> Result<Card, Error> {
    use crate::schema::cards::dsl::*;

    let _ = diesel::insert_into(cards)
        .values(&_card)
        .on_conflict(on_constraint("cards_pkey"))
        .do_update()
        .set((name.eq(_card.name.clone()), card_market_handle.eq(_card.card_market_handle.clone())))
        .execute(conn)
        .expect("Error saving new card");

    for localized_name in _localized_names {
        use crate::schema::localized_names::dsl::*;
        let name_cloned = localized_name.name.clone();
        let _ = diesel::insert_into(localized_names)
            .values(localized_name)
            .on_conflict(on_constraint("localized_names_pkey"))
            .do_update()
            .set(name.eq(name_cloned.clone()))
            .execute(conn)
            .expect("Error saving new localized_name");
    }

    Ok(_card)
}



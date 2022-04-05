use actix_web::{web, Error, HttpResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use std::time::Instant;

use crate::delivery_transaction::execute_delivery;
use crate::models::*;
use crate::new_order_transaction::execute_new_order;
use crate::order_status_transaction::execute_order_status;
use crate::payment_transaction::execute_payment;
use crate::schema::warehouse::dsl::*;
use crate::stock_level_transaction::execute_stock_level;

fn get_all_warehouse(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<Vec<Warehouse>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let warehouses = warehouse.load::<Warehouse>(&conn)?;
    Ok(warehouses)
}

pub async fn get_warehouses(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_warehouse(db))
        .await
        .map(|wh| HttpResponse::Ok().json(wh))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn do_new_order(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    Ok(web::block(move || execute_new_order(db))
        .await
        .map(|_| HttpResponse::Ok().body(format!("{:5?}", now.elapsed())))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn do_payment(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    Ok(web::block(move || execute_payment(db))
        .await
        .map(|_| HttpResponse::Ok().body(format!("{:5?}", now.elapsed())))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn do_order_status(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    Ok(web::block(move || execute_order_status(db))
        .await
        .map(|_| HttpResponse::Ok().body(format!("{:5?}", now.elapsed())))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn do_delivery(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    Ok(web::block(move || execute_delivery(db))
        .await
        .map(|_| HttpResponse::Ok().body(format!("{:5?}", now.elapsed())))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn do_stock_level(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    Ok(web::block(move || execute_stock_level(db))
        .await
        .map(|_| HttpResponse::Ok().body(format!("{:5?}", now.elapsed())))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

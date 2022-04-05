use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

use rust_server::handlers::{
    do_delivery, do_new_order, do_order_status, do_payment, do_stock_level, get_warehouses,
};
use rust_server::services::{echo, hello, home};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let host: String = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8000".to_string(),
    };

    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "postgres://postgres:postgres@localhost:5432".to_string(),
    };

    // Database Connection Pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .service(home)
            .route("api/warehouse", web::get().to(get_warehouses))
            .route("api/new_order", web::get().to(do_new_order))
            .route("api/payment", web::get().to(do_payment))
            .route("api/order_status", web::get().to(do_order_status))
            .route("api/delivery", web::get().to(do_delivery))
            .route("api/stock_level", web::get().to(do_stock_level))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

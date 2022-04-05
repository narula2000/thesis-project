use actix_web::web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{BelongingToDsl, ExpressionMethods};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

use crate::models::*;
use crate::schema::order_line::dsl::*;
use crate::schema::stock::dsl::*;
use crate::schema::warehouse::dsl::*;

pub fn execute_stock_level(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<(), diesel::result::Error> {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    };

    // Get Warehouse
    let warehouses = match warehouse.load::<Warehouse>(&conn) {
        Ok(wh) => wh,
        Err(e) => panic!("{}", e),
    };
    let random_warehouse = match warehouses.choose(&mut rand::thread_rng()) {
        Some(wh) => wh,
        None => panic!("No Warehouse"),
    };

    // Get District
    let districts = match District::belonging_to(random_warehouse).load::<District>(&conn) {
        Ok(dis) => dis,
        Err(e) => panic!("{}", e),
    };
    let random_district = match districts.choose(&mut rand::thread_rng()) {
        Some(dt) => dt,
        None => panic!("No Warehouse"),
    };

    // Get Stock Count
    let some_order_lines = order_line
        .filter(ol_w_id.eq(random_warehouse.w_id))
        .filter(ol_d_id.eq(random_district.d_id))
        .filter(ol_o_id.lt(random_district.d_next_o_id.unwrap()))
        .filter(ol_o_id.gt(random_district.d_next_o_id.unwrap() - 19))
        .get_results::<OrderLine>(&conn)?;

    let rng = rand::thread_rng().gen_range(10..20 + 1);
    let mut ol_i_ids: HashSet<i32> = HashSet::new();
    some_order_lines.iter().for_each(|s_order_line| {
        let some_stocks = stock
            .filter(s_w_id.eq(random_warehouse.w_id))
            .filter(s_i_id.eq(s_order_line.ol_i_id.unwrap()))
            .filter(s_quantity.lt(rng))
            .get_results::<Stock>(&conn);
        let some_stocks = match some_stocks {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
        some_stocks.iter().for_each(|s_stock| {
            ol_i_ids.insert(s_stock.s_i_id);
        })
    });
    Ok(())
}

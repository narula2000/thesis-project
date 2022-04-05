use actix_web::web;
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::models::*;
use crate::schema::customer::dsl::*;
use crate::schema::new_order::dsl::*;
use crate::schema::order_line::dsl::*;
use crate::schema::orders::dsl::*;
use crate::schema::warehouse::dsl::*;

fn take<T>(mut vec: Vec<T>, index: usize) -> Option<T> {
    if vec.get(index).is_none() {
        None
    } else {
        Some(vec.swap_remove(index))
    }
}

pub fn execute_delivery(
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

    // Get 10 District
    for d_id in 1..10 + 1 {
        // Get Min New Order
        let new_orders = new_order
            .filter(no_w_id.eq(random_warehouse.w_id))
            .filter(no_d_id.eq(d_id))
            .order(no_o_id.asc())
            .get_results::<NewOrder>(&conn)?;
        if new_orders.len() < 1 {
            continue;
        }
        let newest_order = match take(new_orders, 0) {
            Some(val) => val,
            None => panic!("No Newest Order"),
        };

        // Get Order
        let some_order = orders
            .filter(o_w_id.eq(random_warehouse.w_id))
            .filter(o_d_id.eq(d_id))
            .filter(o_id.eq(newest_order.no_o_id))
            .get_result::<Order>(&conn)?;

        let some_order_lines = order_line
            .filter(ol_w_id.eq(random_warehouse.w_id))
            .filter(ol_d_id.eq(d_id))
            .filter(ol_o_id.eq(some_order.o_id))
            .get_results::<OrderLine>(&conn)?;

        // Sum Order Line amount
        let mut sum = 0.0;
        some_order_lines.iter().for_each(|s_order_line| {
            sum = sum + s_order_line.ol_amount.unwrap();

            // Update Order Line
            let current_time = Utc::now().naive_utc();
            let _updated_order_line = diesel::update(
                order_line
                    .filter(ol_w_id.eq(random_warehouse.w_id))
                    .filter(ol_d_id.eq(d_id))
                    .filter(ol_o_id.eq(some_order.o_id))
                    .filter(ol_number.eq(s_order_line.ol_number)),
            )
            .set(ol_delivery_d.eq(current_time))
            .get_result::<OrderLine>(&conn);
        });

        // Delet New Order
        let _delete_new_order = diesel::delete(
            new_order
                .filter(no_w_id.eq(random_warehouse.w_id))
                .filter(no_d_id.eq(d_id))
                .filter(no_o_id.eq(newest_order.no_o_id)),
        )
        .get_result::<NewOrder>(&conn);

        // Update Order
        let rng_carrier_id = rand::thread_rng().gen_range(1..10 + 1);
        let updated_order = diesel::update(
            orders
                .filter(o_w_id.eq(random_warehouse.w_id))
                .filter(o_d_id.eq(d_id))
                .filter(o_id.eq(newest_order.no_o_id)),
        )
        .set(o_carrier_id.eq(rng_carrier_id))
        .get_result::<Order>(&conn);
        let updated_order = match updated_order {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        // Get Customer
        let some_customer = customer
            .filter(c_d_id.eq(d_id))
            .filter(c_w_id.eq(random_warehouse.w_id))
            .filter(c_id.eq(updated_order.o_c_id.unwrap()))
            .get_result::<Customer>(&conn)?;

        // Update Customer
        let new_balance = some_customer.c_balance.unwrap() + sum;
        let updated_customer = diesel::update(
            customer
                .filter(c_d_id.eq(d_id))
                .filter(c_w_id.eq(random_warehouse.w_id))
                .filter(c_id.eq(updated_order.o_c_id.unwrap())),
        )
        .set(c_balance.eq(new_balance))
        .get_result::<Customer>(&conn);
        let _updated_customer = match updated_customer {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
    }
    Ok(())
}

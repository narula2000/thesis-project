use actix_web::web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{BelongingToDsl, ExpressionMethods};
use rand::seq::SliceRandom;
use rand::Rng;

use crate::models::*;
use crate::schema::customer::dsl::*;
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

pub fn execute_order_status(
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

    // Get Customer
    let customers = customer
        .filter(c_d_id.eq(random_district.d_id))
        .filter(c_w_id.eq(random_warehouse.w_id))
        .get_results::<Customer>(&conn)?;
    let random_customer = customers.choose(&mut rand::thread_rng()).unwrap();
    let rng: usize = rand::thread_rng().gen_range(1..5 + 1);
    if rng > 2 {
        let l_customers = match customer
            .filter(c_last.eq(random_customer.c_last.as_ref().unwrap()))
            .get_results::<Customer>(&conn)
        {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        let customer_cnt = l_customers.len();
        let middle_cutomer = ((customer_cnt as f32 - 1.0) / 2.0).round() as usize;
        let some_cus = take(l_customers, middle_cutomer).unwrap();

        // Get Last Order
        let some_orders = orders
            .filter(o_w_id.eq(random_warehouse.w_id))
            .filter(o_d_id.eq(random_district.d_id))
            .filter(o_c_id.eq(some_cus.c_id))
            .order(o_id.desc())
            .get_results::<Order>(&conn);
        let some_orders = match some_orders {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        if some_orders.len() > 0 {
            let last_order = match take(some_orders, 0) {
                Some(val) => val,
                None => panic!("No Last Order"),
            };

            // Get Order Line
            let _some_order_line = order_line
                .filter(ol_w_id.eq(last_order.o_w_id))
                .filter(ol_d_id.eq(last_order.o_d_id))
                .filter(ol_o_id.eq(last_order.o_id))
                .get_results::<OrderLine>(&conn);

        }
    } else {
        // Get Last Order
        let some_orders = orders
            .filter(o_w_id.eq(random_warehouse.w_id))
            .filter(o_d_id.eq(random_district.d_id))
            .filter(o_c_id.eq(random_customer.c_id))
            .order(o_id.desc())
            .get_results::<Order>(&conn);
        let some_orders = match some_orders {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        if some_orders.len() > 0 {
            let last_order = match take(some_orders, 0) {
                Some(val) => val,
                None => panic!("No Last Order"),
            };

            // Get Order Line
            let _some_order_line = order_line
                .filter(ol_w_id.eq(last_order.o_w_id))
                .filter(ol_d_id.eq(last_order.o_d_id))
                .filter(ol_o_id.eq(last_order.o_id))
                .get_results::<OrderLine>(&conn);

        }
    }
    Ok(())
}

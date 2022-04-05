use actix_web::web;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{BelongingToDsl, ExpressionMethods};
use rand::seq::SliceRandom;
use rand::Rng;
use std::convert::TryInto;

use crate::models::*;
use crate::schema::customer::dsl::*;
use crate::schema::district::dsl::*;
use crate::schema::item::dsl::*;
use crate::schema::new_order;
use crate::schema::new_order::dsl::*;
use crate::schema::order_line;
use crate::schema::order_line::dsl::*;
use crate::schema::orders;
use crate::schema::orders::dsl::*;
use crate::schema::stock::dsl::*;
use crate::schema::warehouse::dsl::*;

#[derive(Insertable)]
#[table_name = "orders"]
struct CreateOrders<'a> {
    o_id: &'a i32,
    o_d_id: &'a i16,
    o_w_id: &'a i16,
    o_c_id: Option<&'a i32>,
    o_entry_d: &'a NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "new_order"]
struct CreateNewOrder<'a> {
    no_o_id: &'a i32,
    no_d_id: &'a i16,
    no_w_id: &'a i16,
}

#[derive(Insertable)]
#[table_name = "order_line"]
struct CreateOrderLine<'a> {
    ol_o_id: &'a i32,
    ol_d_id: &'a i16,
    ol_w_id: &'a i16,
    ol_number: &'a i32,
    ol_i_id: Option<&'a i32>,
    ol_supply_w_id: Option<&'a i16>,
    ol_delivery_d: Option<&'a NaiveDateTime>,
    ol_quantity: Option<&'a i32>,
    ol_amount: Option<&'a f64>,
}

pub fn execute_new_order(
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

    // Update District
    let next_o_id = match random_district.d_next_o_id {
        Some(val) => val + 1,
        None => panic!("No District next order id"),
    };
    let _updated_district = diesel::update(
        district
            .filter(d_id.eq(random_district.d_id))
            .filter(d_w_id.eq(random_district.d_w_id)),
    )
    .set(d_next_o_id.eq(Some(next_o_id)))
    .get_result::<District>(&conn);

    // Create Order
    let current_time = Utc::now().naive_utc();
    let create_order = CreateOrders {
        o_id: &next_o_id,
        o_d_id: &random_district.d_id,
        o_w_id: &random_warehouse.w_id,
        o_c_id: Some(&random_customer.c_id),
        o_entry_d: &current_time,
    };
    let _created_order = insert_into(orders)
        .values(&create_order)
        .get_result::<Order>(&conn);

    // Create New Order
    let create_new_order = CreateNewOrder {
        no_o_id: &next_o_id,
        no_d_id: &random_district.d_id,
        no_w_id: &random_warehouse.w_id,
    };
    let _created_new_order = insert_into(new_order)
        .values(&create_new_order)
        .get_result::<NewOrder>(&conn);

    // Get Item
    let items = match item.load::<Item>(&conn) {
        Ok(it) => it,
        Err(e) => panic!("{}", e),
    };
    let range: usize = rand::thread_rng().gen_range(5..15 + 1);
    let random_items: Vec<&Item> = items
        .choose_multiple(&mut rand::thread_rng(), range)
        .collect();

    for (idx, r_item) in (random_items).iter().enumerate() {

        // Get Stock
        let s_stock = stock
            .filter(s_i_id.eq(r_item.i_id))
            .filter(s_w_id.eq(random_warehouse.w_id))
            .get_result::<Stock>(&conn)?;

        // Update Stock
        let r_ol_quantity = rand::thread_rng().gen_range(1..10 + 1);
        let new_s_ytd = s_stock.s_ytd.unwrap() + r_ol_quantity;
        let s_quan = if s_stock.s_quantity >= r_ol_quantity + 10 {
            s_stock.s_quantity - r_ol_quantity
        } else {
            s_stock.s_quantity + 91 - r_ol_quantity
        };
        let new_order_cnt = s_stock.s_order_cnt.unwrap() + 1;
        let updated_stock = diesel::update(
            stock
                .filter(s_i_id.eq(r_item.i_id))
                .filter(s_w_id.eq(random_warehouse.w_id)),
        )
        .set((
            s_ytd.eq(Some(new_s_ytd)),
            s_quantity.eq(s_quan),
            s_order_cnt.eq(Some(new_order_cnt)),
        ))
        .get_result::<Stock>(&conn);
        let updated_stock = match updated_stock {
            Ok(st) => st,
            Err(e) => panic!("{}", e),
        };

        // Create OrderLine
        let current_time = Utc::now().naive_utc();
        let new_ol_number: i32 = (idx + 1).try_into().unwrap();
        let new_ol_amount = r_ol_quantity as f64 * r_item.i_price.unwrap();
        let create_order_line = CreateOrderLine {
            ol_o_id: &next_o_id,
            ol_d_id: &random_district.d_id,
            ol_w_id: &random_warehouse.w_id,
            ol_number: &new_ol_number,
            ol_i_id: Some(&r_item.i_id),
            ol_supply_w_id: Some(&updated_stock.s_w_id),
            ol_delivery_d: Some(&current_time),
            ol_quantity: Some(&r_ol_quantity),
            ol_amount: Some(&new_ol_amount),
        };

        let _created_order_line = insert_into(order_line)
            .values(&create_order_line)
            .get_result::<OrderLine>(&conn);
    }

    Ok(())
}

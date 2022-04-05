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

use crate::models::*;
use crate::schema::customer::dsl::*;
use crate::schema::district::dsl::*;
use crate::schema::history;
use crate::schema::history::dsl::*;
use crate::schema::warehouse::dsl::*;

#[derive(Insertable)]
#[table_name = "history"]
struct CreateHistory<'a> {
    h_c_id: Option<&'a i32>,
    h_c_d_id: Option<&'a i16>,
    h_c_w_id: Option<&'a i16>,
    h_d_id: Option<&'a i16>,
    h_w_id: &'a i16,
    h_date: &'a NaiveDateTime,
    h_amount: Option<&'a f64>,
    h_data: Option<&'a String>,
}

fn take<T>(mut vec: Vec<T>, index: usize) -> Option<T> {
    if vec.get(index).is_none() {
        None
    } else {
        Some(vec.swap_remove(index))
    }
}

pub fn execute_payment(
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
    let ware_name = random_warehouse.w_name.as_ref().unwrap();

    // Get District
    let districts = match District::belonging_to(random_warehouse).load::<District>(&conn) {
        Ok(dis) => dis,
        Err(e) => panic!("{}", e),
    };
    let random_district = match districts.choose(&mut rand::thread_rng()) {
        Some(dt) => dt,
        None => panic!("No Warehouse"),
    };
    let dis_name = random_district.d_name.as_ref().unwrap();

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

        // Generate h_amount
        let h_amt: i64 = rand::thread_rng().gen_range(1..5000 + 1);
        let h_amt = h_amt as f64;

        // Update Warehouse
        let _updated_warehouse = diesel::update(warehouse.filter(w_id.eq(random_warehouse.w_id)))
            .set(w_ytd.eq(Some(random_warehouse.w_ytd.unwrap() + h_amt)))
            .get_result::<Warehouse>(&conn);

        // Update District
        let _updated_district = diesel::update(
            district
                .filter(d_id.eq(random_district.d_id))
                .filter(d_w_id.eq(random_district.d_w_id)),
        )
        .set(d_ytd.eq(Some(random_district.d_ytd.unwrap() + h_amt)))
        .get_result::<District>(&conn);

        // Update Customer
        let new_c_balance = some_cus.c_balance.unwrap() - h_amt;
        let new_c_ytd_payment = some_cus.c_ytd_payment.unwrap() + h_amt;
        let new_c_payment_cnt = some_cus.c_payment_cnt.unwrap() + 1;

        // Check for Bad credit
        let credit = if some_cus.c_credit.unwrap() == "BC" {
            let mut cus_data = some_cus.c_data.unwrap();
            let data = format!(
                "{}{}{}{}{}{}",
                some_cus.c_id,
                some_cus.c_d_id,
                some_cus.c_w_id,
                random_district.d_id,
                random_warehouse.w_id,
                h_amt
            );
            let cus_data = if data.len() + cus_data.len() + 1 > 500 {
                format!("{}|{:?}", data, cus_data.truncate(500))
            } else {
                format!("{}|{}", data, cus_data)
            };
            cus_data
        } else {
            "".to_string()
        };

        let _updated_customer = diesel::update(
            customer
                .filter(c_id.eq(some_cus.c_id))
                .filter(c_d_id.eq(random_district.d_id))
                .filter(c_w_id.eq(random_warehouse.w_id)),
        )
        .set((
            c_balance.eq(Some(new_c_balance)),
            c_ytd_payment.eq(Some(new_c_ytd_payment)),
            c_payment_cnt.eq(Some(new_c_payment_cnt)),
            c_data.eq(Some(credit)),
        ))
        .get_result::<Customer>(&conn);

        // Create History
        let hist_data = format!("{}    {}", ware_name, dis_name);
        let current_time = Utc::now().naive_utc();
        let create_history = CreateHistory {
            h_c_id: Some(&some_cus.c_id),
            h_c_d_id: Some(&some_cus.c_d_id),
            h_c_w_id: Some(&some_cus.c_w_id),
            h_d_id: Some(&random_district.d_id),
            h_w_id: &random_warehouse.w_id,
            h_date: &current_time,
            h_amount: Some(&h_amt),
            h_data: Some(&hist_data),
        };

        let _created_history = insert_into(history)
            .values(&create_history)
            .get_result::<History>(&conn);
    } else {
        // Generate h_amount
        let h_amt: i64 = rand::thread_rng().gen_range(1..5000 + 1);
        let h_amt = h_amt as f64;

        // Update Warehouse
        let _updated_warehouse = diesel::update(warehouse.filter(w_id.eq(random_warehouse.w_id)))
            .set(w_ytd.eq(Some(random_warehouse.w_ytd.unwrap() + h_amt)))
            .get_result::<Warehouse>(&conn);

        // Update District
        let _updated_district = diesel::update(
            district
                .filter(d_id.eq(random_district.d_id))
                .filter(d_w_id.eq(random_district.d_w_id)),
        )
        .set(d_ytd.eq(Some(random_district.d_ytd.unwrap() + h_amt)))
        .get_result::<District>(&conn);

        // Update Customer
        let new_c_balance = random_customer.c_balance.unwrap() - h_amt;
        let new_c_ytd_payment = random_customer.c_ytd_payment.unwrap() + h_amt;
        let new_c_payment_cnt = random_customer.c_payment_cnt.unwrap() + 1;

        // Check for Bad credit
        let customer_credit = random_customer.c_credit.as_ref().unwrap();
        let mut cus_data = random_customer.c_data.as_ref().unwrap().clone();
        let credit = if customer_credit == "BC" {
            let data = format!(
                "{}{}{}{}{}{}",
                random_customer.c_id,
                random_customer.c_d_id,
                random_customer.c_w_id,
                random_district.d_id,
                random_warehouse.w_id,
                h_amt
            );
            let cus_data = if data.len() + cus_data.len() + 1 > 500 {
                format!("{}|{:?}", data, cus_data.truncate(500))
            } else {
                format!("{}|{:?}", data, cus_data)
            };
            cus_data
        } else {
            "".to_string()
        };

        let _updated_customer = diesel::update(
            customer
                .filter(c_id.eq(random_customer.c_id))
                .filter(c_d_id.eq(random_district.d_id))
                .filter(c_w_id.eq(random_warehouse.w_id)),
        )
        .set((
            c_balance.eq(Some(new_c_balance)),
            c_ytd_payment.eq(Some(new_c_ytd_payment)),
            c_payment_cnt.eq(Some(new_c_payment_cnt)),
            c_data.eq(Some(credit)),
        ))
        .get_result::<Customer>(&conn);

        // Create History
        let hist_data = format!("{}    {}", ware_name, dis_name);
        let current_time = Utc::now().naive_utc();
        let create_history = CreateHistory {
            h_c_id: Some(&random_customer.c_id),
            h_c_d_id: Some(&random_customer.c_d_id),
            h_c_w_id: Some(&random_customer.c_w_id),
            h_d_id: Some(&random_district.d_id),
            h_w_id: &random_warehouse.w_id,
            h_date: &current_time,
            h_amount: Some(&h_amt),
            h_data: Some(&hist_data),
        };

        let _created_history = insert_into(history)
            .values(&create_history)
            .get_result::<History>(&conn);
    }
    Ok(())
}

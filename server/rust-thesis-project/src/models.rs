#![allow(unused)]
#![allow(clippy::all)]
use super::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(c_w_id, c_d_id, c_id)]
#[belongs_to(District, foreign_key = "c_d_id")]
#[table_name = "customer"]
pub struct Customer {
    pub c_id: i32,
    pub c_d_id: i16,
    pub c_w_id: i16,
    pub c_first: Option<String>,
    pub c_middle: Option<String>,
    pub c_last: Option<String>,
    pub c_street_1: Option<String>,
    pub c_street_2: Option<String>,
    pub c_city: Option<String>,
    pub c_state: Option<String>,
    pub c_zip: Option<String>,
    pub c_phone: Option<String>,
    pub c_since: NaiveDateTime,
    pub c_credit: Option<String>,
    pub c_credit_lim: Option<f64>,
    pub c_discount: Option<f64>,
    pub c_balance: Option<f64>,
    pub c_ytd_payment: Option<f64>,
    pub c_payment_cnt: Option<i32>,
    pub c_delivery_cnt: Option<i32>,
    pub c_data: Option<String>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(d_w_id, d_id)]
#[belongs_to(Warehouse, foreign_key = "d_w_id")]
#[table_name = "district"]
pub struct District {
    pub d_id: i16,
    pub d_w_id: i16,
    pub d_name: Option<String>,
    pub d_street_1: Option<String>,
    pub d_street_2: Option<String>,
    pub d_city: Option<String>,
    pub d_state: Option<String>,
    pub d_zip: Option<String>,
    pub d_tax: Option<f64>,
    pub d_ytd: Option<f64>,
    pub d_next_o_id: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(h_id)]
#[belongs_to(Customer, foreign_key = "h_c_id")]
#[table_name = "history"]
pub struct History {
    pub h_id: i32,
    pub h_c_id: Option<i32>,
    pub h_c_d_id: Option<i16>,
    pub h_c_w_id: Option<i16>,
    pub h_d_id: Option<i16>,
    pub h_w_id: i16,
    pub h_date: NaiveDateTime,
    pub h_amount: Option<f64>,
    pub h_data: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(i_id)]
#[table_name = "item"]
pub struct Item {
    pub i_id: i32,
    pub i_im_id: Option<i32>,
    pub i_name: Option<String>,
    pub i_price: Option<f64>,
    pub i_data: Option<String>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(no_d_id, no_w_id, no_o_id)]
#[belongs_to(Order, foreign_key = "no_o_id")]
#[table_name = "new_order"]
pub struct NewOrder {
    pub no_o_id: i32,
    pub no_d_id: i16,
    pub no_w_id: i16,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(ol_w_id, ol_d_id, ol_o_id, ol_number)]
#[belongs_to(Order, foreign_key = "ol_o_id")]
#[belongs_to(Stock, foreign_key = "ol_i_id")]
#[table_name = "order_line"]
pub struct OrderLine {
    pub ol_o_id: i32,
    pub ol_d_id: i16,
    pub ol_w_id: i16,
    pub ol_number: i32,
    pub ol_i_id: Option<i32>,
    pub ol_supply_w_id: Option<i16>,
    pub ol_delivery_d: Option<NaiveDateTime>,
    pub ol_quantity: Option<i32>,
    pub ol_amount: Option<f64>,
    pub ol_dist_info: Option<String>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(o_w_id, o_d_id, o_id)]
#[belongs_to(Customer, foreign_key = "o_c_id")]
#[table_name = "orders"]
pub struct Order {
    pub o_id: i32,
    pub o_d_id: i16,
    pub o_w_id: i16,
    pub o_c_id: Option<i32>,
    pub o_entry_d: NaiveDateTime,
    pub o_carrier_id: Option<i32>,
    pub o_ol_cnt: Option<i32>,
    pub o_all_local: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[primary_key(s_w_id, s_i_id)]
#[belongs_to(Warehouse, foreign_key = "s_w_id")]
#[table_name = "stock"]
pub struct Stock {
    pub s_i_id: i32,
    pub s_w_id: i16,
    pub s_quantity: i32,
    pub s_dist_01: Option<String>,
    pub s_dist_02: Option<String>,
    pub s_dist_03: Option<String>,
    pub s_dist_04: Option<String>,
    pub s_dist_05: Option<String>,
    pub s_dist_06: Option<String>,
    pub s_dist_07: Option<String>,
    pub s_dist_08: Option<String>,
    pub s_dist_09: Option<String>,
    pub s_dist_10: Option<String>,
    pub s_ytd: Option<i32>,
    pub s_order_cnt: Option<i32>,
    pub s_remote_cnt: Option<i32>,
    pub s_data: Option<String>,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[primary_key(w_id)]
#[table_name = "warehouse"]
pub struct Warehouse {
    pub w_id: i16,
    pub w_name: Option<String>,
    pub w_street_1: Option<String>,
    pub w_street_2: Option<String>,
    pub w_city: Option<String>,
    pub w_state: Option<String>,
    pub w_zip: Option<String>,
    pub w_tax: Option<f64>,
    pub w_ytd: Option<f64>,
}

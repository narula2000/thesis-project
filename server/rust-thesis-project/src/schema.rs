table! {
    customer (c_w_id, c_d_id, c_id) {
        c_id -> Int4,
        c_d_id -> Int2,
        c_w_id -> Int2,
        c_first -> Nullable<Varchar>,
        c_middle -> Nullable<Varchar>,
        c_last -> Nullable<Varchar>,
        c_street_1 -> Nullable<Varchar>,
        c_street_2 -> Nullable<Varchar>,
        c_city -> Nullable<Varchar>,
        c_state -> Nullable<Varchar>,
        c_zip -> Nullable<Varchar>,
        c_phone -> Nullable<Varchar>,
        c_since -> Timestamp,
        c_credit -> Nullable<Varchar>,
        c_credit_lim -> Nullable<Float8>,
        c_discount -> Nullable<Float8>,
        c_balance -> Nullable<Float8>,
        c_ytd_payment -> Nullable<Float8>,
        c_payment_cnt -> Nullable<Int4>,
        c_delivery_cnt -> Nullable<Int4>,
        c_data -> Nullable<Varchar>,
    }
}

table! {
    district (d_w_id, d_id) {
        d_id -> Int2,
        d_w_id -> Int2,
        d_name -> Nullable<Varchar>,
        d_street_1 -> Nullable<Varchar>,
        d_street_2 -> Nullable<Varchar>,
        d_city -> Nullable<Varchar>,
        d_state -> Nullable<Varchar>,
        d_zip -> Nullable<Varchar>,
        d_tax -> Nullable<Float8>,
        d_ytd -> Nullable<Float8>,
        d_next_o_id -> Nullable<Int4>,
    }
}

table! {
    history (h_id) {
        h_id -> Int4,
        h_c_id -> Nullable<Int4>,
        h_c_d_id -> Nullable<Int2>,
        h_c_w_id -> Nullable<Int2>,
        h_d_id -> Nullable<Int2>,
        h_w_id -> Int2,
        h_date -> Timestamp,
        h_amount -> Nullable<Float8>,
        h_data -> Nullable<Varchar>,
    }
}

table! {
    item (i_id) {
        i_id -> Int4,
        i_im_id -> Nullable<Int4>,
        i_name -> Nullable<Varchar>,
        i_price -> Nullable<Float8>,
        i_data -> Nullable<Varchar>,
    }
}

table! {
    new_order (no_d_id, no_w_id, no_o_id) {
        no_o_id -> Int4,
        no_d_id -> Int2,
        no_w_id -> Int2,
    }
}

table! {
    order_line (ol_w_id, ol_d_id, ol_o_id, ol_number) {
        ol_o_id -> Int4,
        ol_d_id -> Int2,
        ol_w_id -> Int2,
        ol_number -> Int4,
        ol_i_id -> Nullable<Int4>,
        ol_supply_w_id -> Nullable<Int2>,
        ol_delivery_d -> Nullable<Timestamp>,
        ol_quantity -> Nullable<Int4>,
        ol_amount -> Nullable<Float8>,
        ol_dist_info -> Nullable<Varchar>,
    }
}

table! {
    orders (o_w_id, o_d_id, o_id) {
        o_id -> Int4,
        o_d_id -> Int2,
        o_w_id -> Int2,
        o_c_id -> Nullable<Int4>,
        o_entry_d -> Timestamp,
        o_carrier_id -> Nullable<Int4>,
        o_ol_cnt -> Nullable<Int4>,
        o_all_local -> Nullable<Int4>,
    }
}

table! {
    stock (s_w_id, s_i_id) {
        s_i_id -> Int4,
        s_w_id -> Int2,
        s_quantity -> Int4,
        s_dist_01 -> Nullable<Varchar>,
        s_dist_02 -> Nullable<Varchar>,
        s_dist_03 -> Nullable<Varchar>,
        s_dist_04 -> Nullable<Varchar>,
        s_dist_05 -> Nullable<Varchar>,
        s_dist_06 -> Nullable<Varchar>,
        s_dist_07 -> Nullable<Varchar>,
        s_dist_08 -> Nullable<Varchar>,
        s_dist_09 -> Nullable<Varchar>,
        s_dist_10 -> Nullable<Varchar>,
        s_ytd -> Nullable<Int4>,
        s_order_cnt -> Nullable<Int4>,
        s_remote_cnt -> Nullable<Int4>,
        s_data -> Nullable<Varchar>,
    }
}

table! {
    warehouse (w_id) {
        w_id -> Int2,
        w_name -> Nullable<Varchar>,
        w_street_1 -> Nullable<Varchar>,
        w_street_2 -> Nullable<Varchar>,
        w_city -> Nullable<Varchar>,
        w_state -> Nullable<Varchar>,
        w_zip -> Nullable<Varchar>,
        w_tax -> Nullable<Float8>,
        w_ytd -> Nullable<Float8>,
    }
}

joinable!(district -> warehouse (d_w_id));
joinable!(stock -> item (s_i_id));
joinable!(stock -> warehouse (s_w_id));

allow_tables_to_appear_in_same_query!(
    customer, district, history, item, new_order, order_line, orders, stock, warehouse,
);

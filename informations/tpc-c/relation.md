# Model Realtion

Warehouse:
  - w_id
District:
  - d_w_id -> w_id
Customer:
  - c_w_id -> d_w_id
  - c_d_id -> d_id
History:
  - h_c_w_id -> c_w_id
  - h_c_d_id -> c_d_id
  - h_c_id -> c_id
  - h_w_id -> d_w_id
  - h_d_id -> d_id
Orders:
  - o_w_id -> c_w_id
  - o_d_id -> c_d_id
  - o_c_id -> c_id
NewOrder:
  - no_w_id -> o_w_id
  - no_d_id -> o_d_id
  - no_o_id -> o_id
Item:
  - i_id
Stock:
  - s_w_id -> w_id
  - s_i_id -> i_id
OrderLine:
  - ol_w_id -> o_w_id
  - ol_d_id -> o_d_id
  - ol_o_id -> o_id
  - ol_supply_w_id -> s_w_id
  - ol_i_id -> s_i_id

const {
  get_random_warehouse,
  get_random_district_from_warehouse,
  get_random_customer_from_warehouse_district,
  get_items,
  random_int
} = require('../utils');

async function do_new_order(models) {
  // Get Warehouse
  const warehouse = await get_random_warehouse(models)
  const w_id = warehouse.w_id

  // Get District
  const district = await get_random_district_from_warehouse(models, warehouse)
  const d_id = district.d_id

  // Get Customer
  const customer = await get_random_customer_from_warehouse_district(models, warehouse, district)

  // Update District
  const next_o_id = district.d_next_o_id + 1
  await models.district.update({d_next_o_id: next_o_id}, {
    where: {
      d_id: d_id,
      d_w_id: w_id
    }
  })

  // Create Order
  const order = await models.orders.create({
    o_id: next_o_id,
    o_d_id: d_id,
    o_w_id: w_id,
    o_c_id: customer.c_id,
  })

  // Create New Order
  await models.new_order.create({
    no_o_id: order.o_id,
    no_d_id: d_id,
    no_w_id: w_id
  })

  const items = await get_items(models)
  const items_length = items.length
  for (let idx = 0; idx < items_length; idx++) {
    const item = items[idx]

    // Get Stock
    const stocks = await models.stock.findAll({
      where: {
        s_i_id: item.i_id,
        s_w_id: w_id
      }
    })
    const stock = stocks[0]
    if (stock == null) {
      return;
    }

    // Update Stock
    const ol_quantity = random_int(1, 10)
    const s_ytd = stock.s_ytd + ol_quantity
    let s_quantity
    if (stock.s_quantity >= ol_quantity + 10) {
      s_quantity = stock.s_quantity - ol_quantity
    } else {
      s_quantity = stock.s_quantity + 91 - ol_quantity
    }
    const s_order_cnt = stock.s_order_cnt + 1
    await models.stock.update({
      s_ytd,
      s_quantity,
      s_order_cnt
    }, {
      where: {
        s_i_id: item.i_id,
        s_w_id: w_id
      }
    })

    // Create Order Line
    const now = new Date().toUTCString();
    await models.order_line.create({
      ol_o_id: order.o_id,
      ol_d_id: d_id,
      ol_w_id: w_id,
      ol_number: idx + 1,
      ol_i_id: stock.s_i_id,
      ol_supply_w_id: stock.s_w_id,
      ol_delivery_d: now,
      ol_quantity: ol_quantity,
      ol_amount: ol_quantity * item.i_price
    })
  }
}

module.exports = do_new_order;

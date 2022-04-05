const {
  get_random_warehouse,
  get_random_district_from_warehouse,
  get_random_customer_from_warehouse_district,
  get_middle_customer,
  random_int
} = require('../utils');

async function do_order_status(models) {
  // Get Warehouse
  const warehouse = await get_random_warehouse(models)
  const w_id = warehouse.w_id

  // Get District
  const district = await get_random_district_from_warehouse(models, warehouse)
  const d_id = district.d_id

  // Get Customer by ID
  let customer = await get_random_customer_from_warehouse_district(models, warehouse, district)

  const rng = random_int(1, 5)
  if (rng > 2) {
    // Get Customer by last name
    const customers = await models.customer.findAll({
      where: {
        c_last: customer.c_last
      }
    })
    customer = get_middle_customer(customers)
  }

  // Get Last Order
  const orders = await models.orders.findAll({
    where: {
      o_w_id: w_id,
      o_d_id: d_id,
      o_c_id: customer.c_id
    },
    order: [
      ['o_id', 'DESC'],
    ]
  })
  if (orders.length > 0) {
    const last_order = orders[0]
    if (last_order == null) {
      return;
    }
    // Get Order Line
    await models.order_line.findAll({
      where: {
        ol_o_id: last_order.o_id,
        ol_d_id: last_order.o_d_id,
        ol_w_id: last_order.o_w_id
      }
    })
  }
}

module.exports = do_order_status;

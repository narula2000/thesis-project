const {
  get_random_warehouse,
  random_int
} = require('../utils');

async function do_delivery(models) {
  // Get Warehouse
  const warehouse = await get_random_warehouse(models)
  const w_id = warehouse.w_id

  // Get 10 District
  for (let d_id = 1; d_id < 11; d_id++) {
    // Get Min New Order
    const new_orders = await models.new_order.findAll({
      where: {
        no_w_id: w_id,
        no_d_id: d_id
      },
      order: [
        ['no_o_id', 'ASC'],
      ]
    })
    const new_order = new_orders[0]
    if (new_order == null) {
      return;
    }

    // Gert Order
    const orders = await models.orders.findAll({
      where: {
        o_d_id: d_id, o_w_id: w_id, o_id: new_order.no_o_id
      }
    })
    const order = orders[0]
    if (order == null) {
      return;
    }

    // Sum Order Line amount
    const order_lines = await models.order_line.findAll({
      where: {
        ol_o_id: order.o_id, ol_d_id: d_id, ol_w_id: w_id
      }
    })
    let order_line_sum = 0
    for (let idx = 0; idx < order_lines.length; idx++) {
      const order_line = order_lines[idx]
      order_line_sum += order_line.ol_amount

      // Update Order Line
      const now = new Date().toUTCString();
      await models.order_line.update({ol_delivery_d: now}, {
        where: {
          ol_w_id: order_line.ol_w_id,
          ol_d_id: order_line.ol_d_id,
          ol_o_id: order_line.ol_o_id,
          ol_number: order_line.ol_number,
        }
      })
    }

    // Delete New Order
    await models.new_order.destroy({
      where: {
        no_w_id: w_id,
        no_d_id: d_id,
        no_o_id: new_order.no_o_id
      }
    })

    // Update Order
    const o_carrier_id = random_int(1, 10)
    await models.orders.update({o_carrier_id}, {
      where: {
        o_w_id: order.o_w_id,
        o_d_id: order.o_d_id,
        o_id: order.o_id
      }
    })

    // Get Customer
    await models.customer.findAll({
      where: {
        c_id: order.o_c_id, c_d_id: d_id, c_w_id: w_id
      }
    })

    // Update Customer
    const c_balance = order_line_sum
    await models.customer.update({c_balance}, {
      where: {
        c_id: order.o_c_id, c_d_id: d_id, c_w_id: w_id
      }
    })
  }
}
module.exports = do_delivery;

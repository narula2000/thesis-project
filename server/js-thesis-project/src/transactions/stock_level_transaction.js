const { Op } = require('sequelize');
const {
  get_random_warehouse,
  get_random_district_from_warehouse,
  random_int
} = require('../utils');

async function do_stock_level(models) {
  // Get Warehouse
  const warehouse = await get_random_warehouse(models)
  const w_id = warehouse.w_id

  // Get District
  const district = await get_random_district_from_warehouse(models, warehouse)
  const d_id = district.d_id
  const o_id = district.d_next_o_id

  // Get Stock Count
  const threshold = random_int(10, 20)
  const order_lines = await models.order_line.findAll({
    where: {
      ol_w_id: w_id, ol_d_id: d_id, ol_o_id: {[Op.lt]: o_id}, ol_o_id: {[Op.gte]: o_id - 20}
    }
  })

  const ol_i_ids = new Set()
  for (let idx = 0; idx < order_lines.length; idx++) {
    const order_line = order_lines[0]
    if (order_line == null) {
      return;
    }
    const stocks = await models.stock.findAll({
      where: {
        s_w_id: w_id, s_i_id: order_line.ol_i_id, s_quantity: {[Op.lt]: threshold}
      }
    })
    stocks.forEach((stock, idx, arr) => {
      ol_i_ids.add(stock.s_i_id)
    })
  }
  ol_i_ids.length
}

module.exports = do_stock_level;

const {
  get_random_warehouse,
  get_random_district_from_warehouse,
  get_random_customer_from_warehouse_district,
  get_middle_customer,
  random_int
} = require('../utils');

async function do_payment(models) {
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

  // Generate h_amount
  const h_amount = random_int(1, 5000)

  // Update Warehouse
  const w_ytd = warehouse.w_ytd + h_amount
  await models.warehouse.update({w_ytd}, {
    where: {
      w_id,
    }
  })

  // Update District
  const d_ytd = district.d_ytd + h_amount
  await models.district.update({d_ytd}, {
    where: {
      d_id: d_id,
      d_w_id: w_id
    }
  })

  // Update Customer
  const c_balance = customer.c_balance - h_amount
  const c_ytd_payment = customer.c_ytd_payment + h_amount
  const c_payment_cnt = customer.c_payment_cnt + 1

  // Check for Bad credit
  let c_data
  if (customer.c_credit == "BC") {
    const new_data = [customer.c_id, customer.c_d_id, customer.c_w_id, d_id, w_id, h_amount].join("")
    c_data = new_data + "|" + customer.c_data
    if (c_data.length > 500){
      c_data = c_data.substring(0,500)
    }
  } else {
    c_data = ""
  }
  await models.customer.update({c_balance, c_ytd_payment, c_payment_cnt, c_data}, {
    where: {
      c_id: customer.c_id,
      c_w_id: customer.c_w_id,
      c_d_id: customer.c_d_id
    }
  })

  // Create History
  const now = new Date().toUTCString();
  const history_data = `${warehouse.w_name}    ${district.d_name}`
  await models.history.create({
    h_c_id: customer.c_id,
    h_c_d_id: customer.c_d_id,
    h_c_w_id: customer.c_w_id,
    h_d_id: d_id,
    h_w_id: w_id,
    h_date: now,
    h_amount: h_amount,
    h_data: history_data
  })
}

module.exports = do_payment;

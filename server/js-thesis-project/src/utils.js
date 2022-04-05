const _ = require('underscore');

function get_random_element(arr) {
  return arr[Math.floor(Math.random() * arr.length)]
}

function random_int(start, end) {
  const min = Math.ceil(start);
  const max = Math.floor(end);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

async function get_random_warehouse(models) {
  const warehouses = await models.warehouse.findAll()
  return get_random_element(warehouses)
}

async function get_random_district_from_warehouse(models, warehouse) {
  const districts = await models.district.findAll({
    where: {
      d_w_id: warehouse.w_id
    }
  })
  return get_random_element(districts)
}

async function get_random_customer_from_warehouse_district(models, warehouse, district) {
  const customers = await models.customer.findAll({
    where: {
      c_w_id: warehouse.w_id,
      c_d_id: district.d_id
    }
  })
  return get_random_element(customers)
}

async function get_items(models) {
  const items = await models.item.findAll()
  const sample = random_int(5, 15)
  return _.sample(items, sample)
}


function get_middle_customer(customers) {
  const customer_count = customers.length
  const middle_customer_index = Math.floor((customer_count - 1) / 2)
  return customers[middle_customer_index]
}

module.exports = {
  get_random_district_from_warehouse,
  get_random_warehouse,
  get_random_customer_from_warehouse_district,
  get_items,
  get_middle_customer,
  random_int
}

const DataTypes = require("sequelize").DataTypes;
const _customer = require("./customer");
const _district = require("./district");
const _history = require("./history");
const _item = require("./item");
const _new_order = require("./new_order");
const _order_line = require("./order_line");
const _orders = require("./orders");
const _stock = require("./stock");
const _warehouse = require("./warehouse");

function initModels(sequelize) {
  const customer = _customer(sequelize, DataTypes);
  const district = _district(sequelize, DataTypes);
  const history = _history(sequelize, DataTypes);
  const item = _item(sequelize, DataTypes);
  const new_order = _new_order(sequelize, DataTypes);
  const order_line = _order_line(sequelize, DataTypes);
  const orders = _orders(sequelize, DataTypes);
  const stock = _stock(sequelize, DataTypes);
  const warehouse = _warehouse(sequelize, DataTypes);

  customer.belongsToMany(customer, {as: 'o_w_id_customers', through: orders, foreignKey: "o_d_id", otherKey: "o_w_id"});
  customer.belongsToMany(customer, {as: 'o_d_id_customers', through: orders, foreignKey: "o_w_id", otherKey: "o_d_id"});
  district.belongsToMany(district, {as: 'c_w_id_districts', through: customer, foreignKey: "c_d_id", otherKey: "c_w_id"});
  district.belongsToMany(district, {as: 'c_d_id_districts', through: customer, foreignKey: "c_w_id", otherKey: "c_d_id"});
  item.belongsToMany(warehouse, {as: 's_w_id_warehouses', through: stock, foreignKey: "s_i_id", otherKey: "s_w_id"});
  warehouse.belongsToMany(item, {as: 's_i_id_items', through: stock, foreignKey: "s_w_id", otherKey: "s_i_id"});
  history.belongsTo(customer, {as: "h_c_d", foreignKey: "h_c_d_id"});
  customer.hasMany(history, {as: "histories", foreignKey: "h_c_d_id"});
  history.belongsTo(customer, {as: "h_c", foreignKey: "h_c_id"});
  customer.hasMany(history, {as: "h_c_histories", foreignKey: "h_c_id"});
  history.belongsTo(customer, {as: "h_c_w", foreignKey: "h_c_w_id"});
  customer.hasMany(history, {as: "h_c_w_histories", foreignKey: "h_c_w_id"});
  orders.belongsTo(customer, {as: "o_c", foreignKey: "o_c_id"});
  customer.hasMany(orders, {as: "orders", foreignKey: "o_c_id"});
  orders.belongsTo(customer, {as: "o_d", foreignKey: "o_d_id"});
  customer.hasMany(orders, {as: "o_d_orders", foreignKey: "o_d_id"});
  orders.belongsTo(customer, {as: "o_w", foreignKey: "o_w_id"});
  customer.hasMany(orders, {as: "o_w_orders", foreignKey: "o_w_id"});
  customer.belongsTo(district, {as: "c_d", foreignKey: "c_d_id"});
  district.hasMany(customer, {as: "customers", foreignKey: "c_d_id"});
  customer.belongsTo(district, {as: "c_w", foreignKey: "c_w_id"});
  district.hasMany(customer, {as: "c_w_customers", foreignKey: "c_w_id"});
  history.belongsTo(district, {as: "h_d", foreignKey: "h_d_id"});
  district.hasMany(history, {as: "histories", foreignKey: "h_d_id"});
  history.belongsTo(district, {as: "h_w", foreignKey: "h_w_id"});
  district.hasMany(history, {as: "h_w_histories", foreignKey: "h_w_id"});
  stock.belongsTo(item, {as: "s_i", foreignKey: "s_i_id"});
  item.hasMany(stock, {as: "stocks", foreignKey: "s_i_id"});
  new_order.belongsTo(orders, {as: "no_d", foreignKey: "no_d_id"});
  orders.hasMany(new_order, {as: "new_orders", foreignKey: "no_d_id"});
  new_order.belongsTo(orders, {as: "no_o", foreignKey: "no_o_id"});
  orders.hasMany(new_order, {as: "no_o_new_orders", foreignKey: "no_o_id"});
  new_order.belongsTo(orders, {as: "no_w", foreignKey: "no_w_id"});
  orders.hasMany(new_order, {as: "no_w_new_orders", foreignKey: "no_w_id"});
  order_line.belongsTo(orders, {as: "ol_d", foreignKey: "ol_d_id"});
  orders.hasMany(order_line, {as: "order_lines", foreignKey: "ol_d_id"});
  order_line.belongsTo(orders, {as: "ol_o", foreignKey: "ol_o_id"});
  orders.hasMany(order_line, {as: "ol_o_order_lines", foreignKey: "ol_o_id"});
  order_line.belongsTo(orders, {as: "ol_w", foreignKey: "ol_w_id"});
  orders.hasMany(order_line, {as: "ol_w_order_lines", foreignKey: "ol_w_id"});
  order_line.belongsTo(stock, {as: "ol_i", foreignKey: "ol_i_id"});
  stock.hasMany(order_line, {as: "order_lines", foreignKey: "ol_i_id"});
  order_line.belongsTo(stock, {as: "ol_supply_w", foreignKey: "ol_supply_w_id"});
  stock.hasMany(order_line, {as: "ol_supply_w_order_lines", foreignKey: "ol_supply_w_id"});
  district.belongsTo(warehouse, {as: "d_w", foreignKey: "d_w_id"});
  warehouse.hasMany(district, {as: "districts", foreignKey: "d_w_id"});
  stock.belongsTo(warehouse, {as: "s_w", foreignKey: "s_w_id"});
  warehouse.hasMany(stock, {as: "stocks", foreignKey: "s_w_id"});

  return {
    customer,
    district,
    history,
    item,
    new_order,
    order_line,
    orders,
    stock,
    warehouse,
  };
}
module.exports = initModels;
module.exports.initModels = initModels;
module.exports.default = initModels;

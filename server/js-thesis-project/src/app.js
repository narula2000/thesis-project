const express = require('express');
const router = express.Router();
const { Sequelize } = require('sequelize');
const path = require('path');
const cookieParser = require('cookie-parser');
const logger = require('morgan');

const do_new_order = require('./transactions/new_order_transaction');
const do_payment = require('./transactions/payment_transaction');
const do_order_status = require('./transactions/order_status_transaction');
const do_delivery = require('./transactions/delivery_transaction');
const do_stock_level = require('./transactions/stock_level_transaction');

const app = express();
const sequelize = new Sequelize('postgres', 'postgres', 'postgres', {
  host: 'localhost',
  dialect: 'postgres'
});

const initModels = require('./models/init-models.js')
const models = initModels(sequelize)

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());
app.use(express.static(path.join(__dirname, 'public')));

app.use('/api/', router.get('/warehouse', async (_, res) => {
  const warehouses = await models.warehouse.findAll()
  res.status(200).json(warehouses)
}));

app.use('/api/', router.get('/new_order', async (_, res) => {
  await do_new_order(models)
  res.send("Done")
}));

app.use('/api/', router.get('/payment', async (_, res) => {
  await do_payment(models)
  res.send("Done")
}));

app.use('/api/', router.get('/order_status', async (_, res) => {
  await do_order_status(models)
  res.send("Done")
}));

app.use('/api/', router.get('/delivery', async (_, res) => {
  await do_delivery(models)
  res.send("Done")
}));

app.use('/api/', router.get('/stock_level', async (_, res) => {
  await do_stock_level(models)
  res.send("Done")
}));

module.exports = app;

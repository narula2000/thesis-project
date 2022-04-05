const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return customer.init(sequelize, DataTypes);
}

class customer extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      c_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true
      },
      c_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'district',
          key: 'd_w_id'
        },
        unique: "customer_c_w_id_c_d_id_c_last_c_first_key"
      },
      c_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'district',
          key: 'd_w_id'
        },
        unique: "customer_c_w_id_c_d_id_c_last_c_first_key"
      },
      c_first: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL",
        unique: "customer_c_w_id_c_d_id_c_last_c_first_key"
      },
      c_middle: {
        type: DataTypes.STRING(2),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_last: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL",
        unique: "customer_c_w_id_c_d_id_c_last_c_first_key"
      },
      c_street_1: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_street_2: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_city: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_state: {
        type: DataTypes.STRING(2),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_zip: {
        type: DataTypes.STRING(9),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_phone: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_since: {
        type: DataTypes.DATE,
        allowNull: false,
        defaultValue: Sequelize.Sequelize.literal('CURRENT_TIMESTAMP')
      },
      c_credit: {
        type: DataTypes.STRING(2),
        allowNull: true,
        defaultValue: "NULL"
      },
      c_credit_lim: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      c_discount: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      c_balance: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      c_ytd_payment: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      c_payment_cnt: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      c_delivery_cnt: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      c_data: {
        type: DataTypes.STRING(500),
        allowNull: true
      }
    }, {
      sequelize,
      tableName: 'customer',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "customer_c_w_id_c_d_id_c_last_c_first_key",
          unique: true,
          fields: [
            {name: "c_w_id"},
            {name: "c_d_id"},
            {name: "c_last"},
            {name: "c_first"},
          ]
        },
        {
          name: "customer_pkey",
          unique: true,
          fields: [
            {name: "c_w_id"},
            {name: "c_d_id"},
            {name: "c_id"},
          ]
        },
        {
          name: "idx_customer",
          fields: [
            {name: "c_w_id"},
            {name: "c_d_id"},
            {name: "c_last"},
          ]
        },
      ]
    });
  }
}

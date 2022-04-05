const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return orders.init(sequelize, DataTypes);
}

class orders extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      o_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        unique: "orders_o_w_id_o_d_id_o_c_id_o_id_key"
      },
      o_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        },
        unique: "orders_o_w_id_o_d_id_o_c_id_o_id_key"
      },
      o_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        },
        unique: "orders_o_w_id_o_d_id_o_c_id_o_id_key"
      },
      o_c_id: {
        type: DataTypes.INTEGER,
        allowNull: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        },
        unique: "orders_o_w_id_o_d_id_o_c_id_o_id_key"
      },
      o_entry_d: {
        type: DataTypes.DATE,
        allowNull: false,
        defaultValue: Sequelize.Sequelize.literal('CURRENT_TIMESTAMP')
      },
      o_carrier_id: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      o_ol_cnt: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      o_all_local: {
        type: DataTypes.INTEGER,
        allowNull: true
      }
    }, {
      sequelize,
      tableName: 'orders',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "idx_orders",
          fields: [
            {name: "o_w_id"},
            {name: "o_d_id"},
            {name: "o_c_id"},
          ]
        },
        {
          name: "orders_o_w_id_o_d_id_o_c_id_o_id_key",
          unique: true,
          fields: [
            {name: "o_w_id"},
            {name: "o_d_id"},
            {name: "o_c_id"},
            {name: "o_id"},
          ]
        },
        {
          name: "orders_pkey",
          unique: true,
          fields: [
            {name: "o_w_id"},
            {name: "o_d_id"},
            {name: "o_id"},
          ]
        },
      ]
    });
  }
}

const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return order_line.init(sequelize, DataTypes);
}

class order_line extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      ol_o_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      },
      ol_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      },
      ol_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      },
      ol_number: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true
      },
      ol_i_id: {
        type: DataTypes.INTEGER,
        allowNull: true,
        references: {
          model: 'stock',
          key: 's_w_id'
        }
      },
      ol_supply_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: true,
        references: {
          model: 'stock',
          key: 's_w_id'
        }
      },
      ol_delivery_d: {
        type: DataTypes.DATE,
        allowNull: true
      },
      ol_quantity: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      ol_amount: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      ol_dist_info: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      }
    }, {
      sequelize,
      tableName: 'order_line',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "idx_order_line_tree",
          fields: [
            {name: "ol_w_id"},
            {name: "ol_d_id"},
            {name: "ol_o_id"},
          ]
        },
        {
          name: "order_line_pkey",
          unique: true,
          fields: [
            {name: "ol_w_id"},
            {name: "ol_d_id"},
            {name: "ol_o_id"},
            {name: "ol_number"},
          ]
        },
      ]
    });
  }
}

const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return stock.init(sequelize, DataTypes);
}

class stock extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      s_i_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'item',
          key: 'i_id'
        }
      },
      s_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'warehouse',
          key: 'w_id'
        }
      },
      s_quantity: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0
      },
      s_dist_01: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_02: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_03: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_04: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_05: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_06: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_07: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_08: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_09: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_dist_10: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      s_ytd: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      s_order_cnt: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      s_remote_cnt: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      s_data: {
        type: DataTypes.STRING(64),
        allowNull: true,
        defaultValue: "NULL"
      }
    }, {
      sequelize,
      tableName: 'stock',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "stock_pkey",
          unique: true,
          fields: [
            {name: "s_w_id"},
            {name: "s_i_id"},
          ]
        },
      ]
    });
  }
}

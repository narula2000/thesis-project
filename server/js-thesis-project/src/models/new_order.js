const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return new_order.init(sequelize, DataTypes);
}

class new_order extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      no_o_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      },
      no_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      },
      no_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'orders',
          key: 'o_w_id'
        }
      }
    }, {
      sequelize,
      tableName: 'new_order',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "no_pk_tree",
          unique: true,
          fields: [
            {name: "no_d_id"},
            {name: "no_w_id"},
            {name: "no_o_id"},
          ]
        },
      ]
    });
  }
}

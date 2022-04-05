const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return history.init(sequelize, DataTypes);
}

class history extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      h_id: {
        autoIncrement: true,
        type: DataTypes.INTEGER,
        allowNull: false,
        primaryKey: true
      },
      h_c_id: {
        type: DataTypes.INTEGER,
        allowNull: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        }
      },
      h_c_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        }
      },
      h_c_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: true,
        references: {
          model: 'customer',
          key: 'c_w_id'
        }
      },
      h_d_id: {
        type: DataTypes.SMALLINT,
        allowNull: true,
        references: {
          model: 'district',
          key: 'd_w_id'
        }
      },
      h_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        references: {
          model: 'district',
          key: 'd_w_id'
        }
      },
      h_date: {
        type: DataTypes.DATE,
        allowNull: false,
        defaultValue: Sequelize.Sequelize.literal('CURRENT_TIMESTAMP')
      },
      h_amount: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      h_data: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      }
    }, {
      sequelize,
      tableName: 'history',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "history_pkey",
          unique: true,
          fields: [
            {name: "h_id"},
          ]
        },
      ]
    });
  }
}

const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return district.init(sequelize, DataTypes);
}

class district extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      d_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true
      },
      d_w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true,
        references: {
          model: 'warehouse',
          key: 'w_id'
        }
      },
      d_name: {
        type: DataTypes.STRING(16),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_street_1: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_street_2: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_city: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_state: {
        type: DataTypes.STRING(2),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_zip: {
        type: DataTypes.STRING(9),
        allowNull: true,
        defaultValue: "NULL"
      },
      d_tax: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      d_ytd: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      d_next_o_id: {
        type: DataTypes.INTEGER,
        allowNull: true
      }
    }, {
      sequelize,
      tableName: 'district',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "district_pkey",
          unique: true,
          fields: [
            {name: "d_w_id"},
            {name: "d_id"},
          ]
        },
      ]
    });
  }
}

const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return warehouse.init(sequelize, DataTypes);
}

class warehouse extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      w_id: {
        type: DataTypes.SMALLINT,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true
      },
      w_name: {
        type: DataTypes.STRING(16),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_street_1: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_street_2: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_city: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_state: {
        type: DataTypes.STRING(2),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_zip: {
        type: DataTypes.STRING(9),
        allowNull: true,
        defaultValue: "NULL"
      },
      w_tax: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      w_ytd: {
        type: DataTypes.DOUBLE,
        allowNull: true
      }
    }, {
      sequelize,
      tableName: 'warehouse',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "w_pk_array",
          unique: true,
          fields: [
            {name: "w_id"},
          ]
        },
      ]
    });
  }
}

const { Sequelize } = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  return item.init(sequelize, DataTypes);
}

class item extends Sequelize.Model {
  static init(sequelize, DataTypes) {
    return super.init({
      i_id: {
        type: DataTypes.INTEGER,
        allowNull: false,
        defaultValue: 0,
        primaryKey: true
      },
      i_im_id: {
        type: DataTypes.INTEGER,
        allowNull: true
      },
      i_name: {
        type: DataTypes.STRING(32),
        allowNull: true,
        defaultValue: "NULL"
      },
      i_price: {
        type: DataTypes.DOUBLE,
        allowNull: true
      },
      i_data: {
        type: DataTypes.STRING(64),
        allowNull: true,
        defaultValue: "NULL"
      }
    }, {
      sequelize,
      tableName: 'item',
      schema: 'public',
      timestamps: false,
      indexes: [
        {
          name: "i_pk_array",
          unique: true,
          fields: [
            {name: "i_id"},
          ]
        },
      ]
    });
  }
}

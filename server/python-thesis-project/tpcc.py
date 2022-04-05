from django.db import models

class Customer(models.Model):
    c_id = models.IntegerField()
    c_d = models.ForeignKey('District', models.DO_NOTHING)
    c_w_id = models.SmallIntegerField(primary_key=True)
    c_first = models.CharField(max_length=32, blank=True, null=True)
    c_middle = models.CharField(max_length=2, blank=True, null=True)
    c_last = models.CharField(max_length=32, blank=True, null=True)
    c_street_1 = models.CharField(max_length=32, blank=True, null=True)
    c_street_2 = models.CharField(max_length=32, blank=True, null=True)
    c_city = models.CharField(max_length=32, blank=True, null=True)
    c_state = models.CharField(max_length=2, blank=True, null=True)
    c_zip = models.CharField(max_length=9, blank=True, null=True)
    c_phone = models.CharField(max_length=32, blank=True, null=True)
    c_since = models.DateTimeField()
    c_credit = models.CharField(max_length=2, blank=True, null=True)
    c_credit_lim = models.FloatField(blank=True, null=True)
    c_discount = models.FloatField(blank=True, null=True)
    c_balance = models.FloatField(blank=True, null=True)
    c_ytd_payment = models.FloatField(blank=True, null=True)
    c_payment_cnt = models.IntegerField(blank=True, null=True)
    c_delivery_cnt = models.IntegerField(blank=True, null=True)
    c_data = models.CharField(max_length=500, blank=True, null=True)

    class Meta:
        db_table = 'customer'
        unique_together = (('c_w_id', 'c_d', 'c_id'), ('c_w_id', 'c_d', 'c_last', 'c_first'),)


class District(models.Model):
    d_id = models.SmallIntegerField()
    d_w = models.OneToOneField('Warehouse', models.DO_NOTHING, primary_key=True)
    d_name = models.CharField(max_length=16, blank=True, null=True)
    d_street_1 = models.CharField(max_length=32, blank=True, null=True)
    d_street_2 = models.CharField(max_length=32, blank=True, null=True)
    d_city = models.CharField(max_length=32, blank=True, null=True)
    d_state = models.CharField(max_length=2, blank=True, null=True)
    d_zip = models.CharField(max_length=9, blank=True, null=True)
    d_tax = models.FloatField(blank=True, null=True)
    d_ytd = models.FloatField(blank=True, null=True)
    d_next_o_id = models.IntegerField(blank=True, null=True)

    class Meta:
        db_table = 'district'
        unique_together = (('d_w', 'd_id'),)

class History(models.Model):
    h_id = models.BigAutoField(primary_key=True)
    h_c = models.ForeignKey(Customer, models.DO_NOTHING, blank=True, null=True)
    h_c_d_id = models.SmallIntegerField(blank=True, null=True)
    h_c_w_id = models.SmallIntegerField(blank=True, null=True)
    h_d = models.ForeignKey(District, models.DO_NOTHING, blank=True, null=True)
    h_w_id = models.SmallIntegerField()
    h_date = models.DateTimeField()
    h_amount = models.FloatField(blank=True, null=True)
    h_data = models.CharField(max_length=32, blank=True, null=True)

    class Meta:
        db_table = 'history'


class Item(models.Model):
    i_id = models.IntegerField(primary_key=True)
    i_im_id = models.IntegerField(blank=True, null=True)
    i_name = models.CharField(max_length=32, blank=True, null=True)
    i_price = models.FloatField(blank=True, null=True)
    i_data = models.CharField(max_length=64, blank=True, null=True)

    class Meta:
        db_table = 'item'


class NewOrder(models.Model):
    no_o = models.ForeignKey('Orders', models.DO_NOTHING)
    no_d_id = models.SmallIntegerField(primary_key=True)
    no_w_id = models.SmallIntegerField()

    class Meta:
        db_table = 'new_order'
        unique_together = (('no_d_id', 'no_w_id', 'no_o'),)


class OrderLine(models.Model):
    ol_o = models.ForeignKey('Orders', models.DO_NOTHING)
    ol_d_id = models.SmallIntegerField()
    ol_w_id = models.SmallIntegerField(primary_key=True)
    ol_number = models.IntegerField()
    ol_i = models.ForeignKey('Stock', models.DO_NOTHING, blank=True, null=True)
    ol_supply_w_id = models.SmallIntegerField(blank=True, null=True)
    ol_delivery_d = models.DateTimeField(blank=True, null=True)
    ol_quantity = models.IntegerField(blank=True, null=True)
    ol_amount = models.FloatField(blank=True, null=True)
    ol_dist_info = models.CharField(max_length=32, blank=True, null=True)

    class Meta:
        db_table = 'order_line'
        unique_together = (('ol_w_id', 'ol_d_id', 'ol_o', 'ol_number'),)


class Orders(models.Model):
    o_id = models.IntegerField()
    o_d_id = models.SmallIntegerField()
    o_w_id = models.SmallIntegerField(primary_key=True)
    o_c = models.ForeignKey(Customer, models.DO_NOTHING, blank=True, null=True)
    o_entry_d = models.DateTimeField()
    o_carrier_id = models.IntegerField(blank=True, null=True)
    o_ol_cnt = models.IntegerField(blank=True, null=True)
    o_all_local = models.IntegerField(blank=True, null=True)

    class Meta:
        db_table = 'orders'
        unique_together = (('o_w_id', 'o_d_id', 'o_id'), ('o_w_id', 'o_d_id', 'o_c', 'o_id'),)


class Stock(models.Model):
    s_i = models.ForeignKey(Item, models.DO_NOTHING)
    s_w = models.OneToOneField('Warehouse', models.DO_NOTHING, primary_key=True)
    s_quantity = models.IntegerField()
    s_dist_01 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_02 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_03 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_04 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_05 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_06 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_07 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_08 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_09 = models.CharField(max_length=32, blank=True, null=True)
    s_dist_10 = models.CharField(max_length=32, blank=True, null=True)
    s_ytd = models.IntegerField(blank=True, null=True)
    s_order_cnt = models.IntegerField(blank=True, null=True)
    s_remote_cnt = models.IntegerField(blank=True, null=True)
    s_data = models.CharField(max_length=64, blank=True, null=True)

    class Meta:
        db_table = 'stock'
        unique_together = (('s_w', 's_i'),)


class Warehouse(models.Model):
    w_id = models.SmallIntegerField(primary_key=True)
    w_name = models.CharField(max_length=16, blank=True, null=True)
    w_street_1 = models.CharField(max_length=32, blank=True, null=True)
    w_street_2 = models.CharField(max_length=32, blank=True, null=True)
    w_city = models.CharField(max_length=32, blank=True, null=True)
    w_state = models.CharField(max_length=2, blank=True, null=True)
    w_zip = models.CharField(max_length=9, blank=True, null=True)
    w_tax = models.FloatField(blank=True, null=True)
    w_ytd = models.FloatField(blank=True, null=True)

    class Meta:
        db_table = 'warehouse'

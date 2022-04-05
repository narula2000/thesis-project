import random

from django.utils import timezone

import api.utils as utils
from api.models import NewOrder, OrderLine, Orders, Stock


def do_new_order():
    # Get Warehouse
    warehouse = utils.get_random_warehouse()
    w_id = warehouse.w_id

    # Get District
    district = utils.get_random_district_from_warehouse(warehouse)
    d_id = district.d_id

    # Get Customer
    customer = utils.get_random_customer_from_warehouse_and_district(
        warehouse, district)

    # Update District
    district.d_next_o_id += 1
    district.save(update_fields=['d_next_o_id'])

    # Create Order
    order = Orders.objects.create(
        o_id=district.d_next_o_id,
        o_d_id=d_id,
        o_w_id=w_id,
        o_c=customer,
        o_entry_d=timezone.now(),
    )

    # Create New Order
    NewOrder.objects.create(
        no_o_id=order.o_id,
        no_d_id=d_id,
        no_w_id=w_id
    )

    # Get Item
    items = utils.get_items()
    for idx, item in enumerate(items):
        # Get Stock
        stock = Stock.objects.get(s_i_id=item.i_id, s_w__w_id=w_id)

        # Update Stock
        ol_quantity = random.randint(1, 10)
        stock.s_ytd += ol_quantity
        if stock.s_quantity >= ol_quantity + 10:
            stock.s_quantity -= ol_quantity
        else:
            stock.s_quantity += 91 - ol_quantity
        stock.s_order_cnt += 1
        stock.save(update_fields=['s_ytd', 's_quantity', 's_order_cnt'])

        # Create OrderLine
        OrderLine.objects.create(
            ol_o_id=order.o_id,
            ol_d_id=d_id,
            ol_w_id=w_id,
            ol_number=idx+1,
            ol_i=stock,
            ol_supply_w_id=stock.s_w.w_id,
            ol_delivery_d=timezone.now(),
            ol_quantity=ol_quantity,
            ol_amount=ol_quantity*item.i_price
        )

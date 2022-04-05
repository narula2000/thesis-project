import random

from django.utils import timezone

import api.utils as utils
from api.models import Customer, NewOrder, OrderLine, Orders


def do_delivery():
    # Get Warehouse
    warehouse = utils.get_random_warehouse()
    w_id = warehouse.w_id

    # Get 10 District
    for d_id in range(1, 11):
        # Get Min New Order
        new_order = NewOrder.objects.filter(
            no_w_id=w_id, no_d_id=d_id).order_by('no_o_id')
        if len(new_order) < 1:
            continue

        new_order = new_order[0]
        # Get Order
        order = Orders.objects.get(
            o_d_id=d_id, o_w_id=w_id, o_id=new_order.no_o_id)

        # Sum Order Line amount
        order_lines = OrderLine.objects.filter(
            ol_o_id=order.o_id, ol_d_id=d_id, ol_w_id=w_id)
        order_line_amount_sum = 0
        for order_line in order_lines:
            order_line_amount_sum += order_line.ol_amount

            # Update Order Line
            order_line.ol_delivery_d = timezone.now()
            order_line.save(update_fields=['ol_delivery_d'])

        # Delete New Order
        new_order.delete()

        # Update Order
        carrier_id = random.randint(1, 10)
        order.o_carrier_id = carrier_id
        order.save(update_fields=['o_carrier_id'])

        # Get Customer
        customer = Customer.objects.get(c_id=order.o_c_id, c_d_id=d_id, c_w_id=w_id)

        # Update Customer
        customer.c_balance += order_line_amount_sum
        customer.save(update_fields=['c_balance'])

import random

import api.utils as utils
from api.models import Customer, OrderLine, Orders


def do_order_status():
    # Get Warehouse
    warehouse = utils.get_random_warehouse()
    w_id = warehouse.w_id

    # Get District
    district = utils.get_random_district_from_warehouse(warehouse)
    d_id = district.d_id

    # Get Customer by ID
    customer = utils.get_random_customer_from_warehouse_and_district(
        warehouse, district)
    rng = random.randint(1, 5)
    if rng > 2:  # 60% of the time
        # Get Customer by Last name
        customers = list(Customer.objects.filter(c_last=customer.c_last))
        customer = utils.get_middle_customer(customers)

    # Get Last Order
    orders = Orders.objects.filter(
        o_w_id=w_id, o_d_id=d_id, o_c_id=customer.c_id).order_by('-o_id')
    if len(orders) > 0:
        last_order = orders[0]
        # Get Order Line
        OrderLine.objects.filter(
            ol_o_id=last_order.o_id,
            ol_d_id=last_order.o_d_id,
            ol_w_id=last_order.o_w_id
        )

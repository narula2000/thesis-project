import random

from django.utils import timezone

import api.utils as utils
from api.models import Customer, History


def do_payment():
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

    # Generate h_amount
    h_amount = random.randint(1, 5000)

    # Update Warehouse
    warehouse.w_ytd += h_amount
    warehouse.save(update_fields=['w_ytd'])

    # Update District
    district.d_ytd += h_amount
    district.save(update_fields=['d_ytd'])

    # Update Customer
    customer.c_balance -= h_amount
    customer.c_ytd_payment += h_amount
    customer.c_payment_cnt += 1

    # Check for Bad credit
    if customer.c_credit == "BC":
        new_data = "".join(
            map(str,
                [customer.c_id, customer.c_d_id, customer.c_w_id,
                 d_id, w_id, h_amount]))
        customer_data = new_data + "|" + customer.c_data
        if len(customer_data) > 500:
            customer_data = customer_data[:500]
        customer.c_data = customer_data
    else:
        customer.c_data = ""
    customer.save(update_fields=['c_balance', 'c_ytd_payment', 'c_payment_cnt', 'c_data'])

    # Create History
    history_data = f'{warehouse.w_name}    {district.d_name}'
    History.objects.create(
        h_c_id=customer.c_id,
        h_c_d_id=customer.c_d_id,
        h_c_w_id=customer.c_w_id,
        h_d_id=district.d_id,
        h_w_id=warehouse.w_id,
        h_date=timezone.now(),
        h_amount=h_amount,
        h_data=history_data
    )

import random

import api.utils as utils
from api.models import OrderLine, Stock


def do_stock_level():
    # Get Warehouse
    warehouse = utils.get_random_warehouse()
    w_id = warehouse.w_id

    # Get District
    district = utils.get_random_district_from_warehouse(warehouse)
    d_id = district.d_id
    o_id = district.d_next_o_id

    # Get Stock Count
    order_lines = OrderLine.objects.filter(
        ol_w_id=w_id, ol_d_id=d_id, ol_o_id__lt=o_id, ol_o_id__gte=o_id-20)
    threshold = random.randint(10, 20)
    ol_i_ids = set()
    for order_line in order_lines:
        stocks = Stock.objects.filter(
            s_w_id=w_id, s_i_id=order_line.ol_i_id, s_quantity__lt=threshold)
        for stock in stocks:
            ol_i_ids.add(stock.s_i_id)
    len(ol_i_ids)
    return None

import random

from api.models import Customer, District, Item, Warehouse


def get_random_warehouse():
    list_warehouses = list(Warehouse.objects.all())
    return random.choice(list_warehouses)


def get_random_district_from_warehouse(warehouse):
    list_districts = list(District.objects.filter(d_w=warehouse))
    return random.choice(list_districts)


def get_random_customer_from_warehouse_and_district(warehouse, district):
    list_customers = list(Customer.objects.filter(
        c_d_id=district.d_id, c_w_id=warehouse.w_id))
    return random.choice(list_customers)


def get_items():
    list_items = list(Item.objects.all())
    sample = random.randint(5, 15)
    return random.sample(list_items, sample)

def get_middle_customer(customers):
    customer_count = len(customers)
    middle_customer_index = int((customer_count - 1) / 2)
    return customers[middle_customer_index]

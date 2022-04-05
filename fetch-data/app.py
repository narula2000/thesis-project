import csv
import requests
import os

ip = os.getenv('ip', "127.0.0.1")

def get_req_time(api, transaction):
    url = f'{api}/{transaction}'
    res = requests.get(url, timeout=5000)
    return res.elapsed.total_seconds()

def make_csv(headers, data, fname='out.csv',):
    with open(fname, 'w') as file:
        write = csv.writer(file)
        write.writerow(headers)
        for tup in zip(*data):
            print("-->", tup)
            write.writerow(tup)

if __name__ == '__main__':
    query_amount = 100
    fname = 'out.csv'
    api = f"http://{ip}:8000/api"
    headers = ["new_order", "payment", "order_status", "delivery", "stock_level", "total_time"]
    transactions = headers[:-1]
    times = []
    for idx in range(query_amount):
        req_times = []
        for transaction in transactions:
            req_times.append(get_req_time(api, transaction))
        req_times.append(sum(req_times))
        print("-> total_time", "\t",idx, "\t",req_times[-1])
        times.append(tuple(req_times))

    new_orders = (t[0] for t in times)
    payments = (t[1] for t in times)
    order_statuses = (t[2] for t in times)
    deliverys = (t[3] for t in times)
    stock_levels = (t[4] for t in times)
    total_time = (t[5] for t in times)

    data_rows = [new_orders, payments, order_statuses, deliverys, stock_levels, total_time]

    make_csv(headers, data_rows, fname)

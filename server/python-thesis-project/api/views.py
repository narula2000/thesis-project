import time

from django.http.response import HttpResponseNotAllowed
from rest_framework.decorators import api_view
from rest_framework.response import Response

from api.models import Warehouse
from api.serializers import WarehouseSerializer
from api.transactions.delivery_transaction import do_delivery
from api.transactions.new_order_transaction import do_new_order
from api.transactions.order_status_transaction import do_order_status
from api.transactions.payment_transaction import do_payment
from api.transactions.stock_level_transaction import do_stock_level


@api_view(['GET'])
def get_warehouse(request):
    if request.method == 'GET':
        query = Warehouse.objects.all()
        serializers = WarehouseSerializer(query, many=True)
        return Response(serializers.data)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)


@api_view(['GET'])
def new_order(request):
    if request.method == 'GET':
        start = time.time()
        do_new_order()
        total_time = time.time() - start
        return Response(total_time)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)


@api_view(['GET'])
def payment(request):
    if request.method == 'GET':
        start = time.time()
        do_payment()
        total_time = time.time() - start
        return Response(total_time)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)


@api_view(['GET'])
def order_status(request):
    if request.method == 'GET':
        start = time.time()
        do_order_status()
        total_time = time.time() - start
        return Response(total_time)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)


@api_view(['GET'])
def delivery(request):
    if request.method == 'GET':
        start = time.time()
        do_delivery()
        total_time = time.time() - start
        return Response(total_time)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)


@api_view(['GET'])
def stock_level(request):
    if request.method == 'GET':
        start = time.time()
        do_stock_level()
        total_time = time.time() - start
        return Response(total_time)
    else:
        return HttpResponseNotAllowed("Please use GET method", status=405)

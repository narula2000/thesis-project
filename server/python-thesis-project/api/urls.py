from django.urls import path
from . import views

urlpatterns = [
    path('warehouse', views.get_warehouse, name="get-warehouse"),
    path('new_order', views.new_order, name="new-order"),
    path('payment', views.payment, name="payment"),
    path('order_status', views.order_status, name="order-status"),
    path('delivery', views.delivery, name="delivery"),
    path('stock_level', views.stock_level, name="stock-level"),
]

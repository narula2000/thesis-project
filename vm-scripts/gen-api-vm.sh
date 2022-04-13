#!/bin/bash

virt-install \
  --name ubuntu \
  --vcpus 4 \
  --ram 8192 \
  --disk path=u20.qcow2,size=15 \
  --console pty,target_type=serial \
  --cdrom ubuntu-20.04.4-live-server-amd64.iso

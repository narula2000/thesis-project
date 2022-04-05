#!/bin/bash

virt-install \
  --name database \
  --vcpus 4 \
  --ram 8192 \
  --disk path=db.qcow2,size=45 \
  --console pty,target_type=serial \
  --cdrom ubuntu-20.04.4-live-server-amd64.iso

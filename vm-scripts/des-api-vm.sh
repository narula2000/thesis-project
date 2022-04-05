#!/bin/bash

virsh list --all
virsh shutdown ubuntu
virsh destroy ubuntu
virsh undefine ubuntu --remove-all-storage
virsh list --all

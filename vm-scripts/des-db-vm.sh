#!/bin/bash

virsh list --all
virsh shutdown database
virsh destroy database
virsh undefine database --remove-all-storage
virsh list --all

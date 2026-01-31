#!/bin/bash
_ip=$1
key="key.pem"

sudo ssh admin@$_ip -i key.pem
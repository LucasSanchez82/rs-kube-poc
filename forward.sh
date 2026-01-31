#!/bin/bash
_ip=$1
key="key.pem"


sudo ssh -L 8443:localhost:32771 admin@$_ip -i key.pem
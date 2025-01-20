#!/bin/bash

curl  -v -i \
    --request POST \
    -H "Content-Type: application/json" \
    -d @preprocess-image/image_data.json \
    http://tf-inference.sc2.192.168.1.230.sslip.io/v1/models/mobilenet:predict

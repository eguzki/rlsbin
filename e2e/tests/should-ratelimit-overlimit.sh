#!/bin/bash

response=$(curl -i -s http://envoy/ratelimit-overlimit | grep "Too Many Requests")

if [ $? -ne 0 ]; then
	echo "This should trigger a ratelimit"
	exit 1
fi

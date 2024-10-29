#!/bin/bash

response=$(curl -f -s http://envoy/ratelimit-ok)

if [ $? -ne 0 ]; then
	echo "Rate limit should not trigger yet"
	exit 1
fi

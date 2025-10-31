#!/bin/sh

curl -N -X POST http://localhost:6767/run -H "Content-Type: application/json" -H "Accept: text/event-stream" -d "{\"src_path\": \"$1\", \"dst_path\": \"main\"}"

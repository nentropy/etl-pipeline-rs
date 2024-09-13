#!/bin/bash

/usr/share/grafana/bin/grafana-server &

/bin/prometheus --config.file=/etc/prometheus/prometheus.yml --storage.tsdb.path=/prometheus &

/usr/bin/loki --config.file=/etc/loki/local-config.yaml &

/usr/bin/tempo --config.file=/etc/tempo/config.yaml &

wait
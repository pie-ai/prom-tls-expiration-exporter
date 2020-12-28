#!/bin/bash
docker build . -t pieai/prom-tls-expiration-exporter:arm64
systemctl restart docker-compose-prometheus.service

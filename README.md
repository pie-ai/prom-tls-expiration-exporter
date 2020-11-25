# prometheus exporter that analyses tls expiration

## docker

```
$ cat endpoints.csv
host,port
google.com,443
pop.gmail.com,995
imap.gmail.com,993
```

```
$ docker run -v "$(pwd)"/test.csv:/endpoints.csv pieai/prom-tls-expiration-exporter /rust-binary --endpoints /endpoints.csv
[20XX-XX-XXTXX:XX:XXZ INFO  prom_tls_expiration_exporter] starting exporter on 0.0.0.0:6661
[20XX-XX-XXTXX:XX:XXZ INFO  prometheus_exporter_base] Listening on http://0.0.0.0:6661
```

```
$ curl http://127.0.0.1:6661/metrics
# HELP tls_expiration expiration of tls certificates
# TYPE tls_expiration summary
tls_expiration{host="google.com",port="443"} 3322
tls_expiration{host="pop.gmail.com",port="995"} 3322
tls_expiration{host="imap.gmail.com",port="993"} 3322
```

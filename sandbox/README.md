## Sandbox development environment

### Envoy logs

```
docker compose --project-directory $PWD -f sandbox/docker-compose.yaml logs -f envoy
```

### RLSBIN logs

```
docker compose --project-directory $PWD -f sandbox/docker-compose.yaml logs -f rlsbin
```

### ratelimit-ok

```
curl --silent -v --fail --resolve example.com:18000:127.0.0.1 "http://example.com:18000/ratelimit-ok"
```



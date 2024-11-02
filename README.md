# RateLimit Mock Service

Rate limiting mock service that integrates with [Envoy's RLS protocol](https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/ratelimit/v3/rls.proto).

## FEATURES

### Response code

By default, *rlsbin* will return `Code::Ok`. You can override the response code value sending
`x-overlimit` RLS descriptor entry key, regardless of the descriptor entry value.

Using [Envoy rate limiting filter](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/rate_limit_filter)

```yaml
route_config:
  name: local_route
  virtual_hosts:
    - name: local_service
      domains:
        - "*"
      routes:
        - match:
            prefix: "/ratelimit-overlimit"
          route:
            cluster: upstream
            rate_limits:
            - actions:
              - generic_key:
                  descriptor_key: x-overlimit
                  descriptor_value: "a"
```

Using [grpcurl](https://github.com/fullstorydev/grpcurl)

```bash
bin/grpcurl -plaintext -d @ 127.0.0.1:8081 envoy.service.ratelimit.v3.RateLimitService.ShouldRateLimit <<EOM
{
    "domain": "a",
    "hits_addend": 1,
    "descriptors": [
        {
            "entries": [
                {
                    "key": "x-overlimit",
                    "value": "a"
                }
            ]
        }
    ]
}
EOM
```

> Note: This *grpcurl* command does not get the proto sources, so either provide those RLS proto sources or make sure *rlsbin* is started with the reflection API enabled with the `--grpc-reflection-service` command line option

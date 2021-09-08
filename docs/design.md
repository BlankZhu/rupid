# Rupid API Gateway

## Architecture

```
1. Acceptor
2. Router(Handler)
3. Filters(Filter Chain, or Route)
4. Proxy
5. Upstream Services
```

### Acceptor

`Acceptor` listens to a port, accepts HTTP request and maps the request target to a `Router`.

Acceptor should be able to handle HTTPS request from client.

### Router(Handler)

`Router` (or handler) will check if the HTTP request is valid, create `RupidContext`, call the global `filter` chain and the custom filter chain.

### Filters(Filter Chain)

`Filters` will pre-process the request, or intercept, or post-process the response, which works like a HTTP middleware but with a lot of combinable logics.

`PredicateFilter` will handle the request/response according to its `Predicate`.

`ThrottleFilter` will throttle the request.

`CircuitBreakerFilter` will make a service-downgrade if upstream services are not in good condition.

### Proxy

`Proxy` will finally get the pre-processed request from filter chains, do the proxy jobs to upstream services and return back the response.

`Proxy` should be able to handle HTTPS to the upstream services.

### Upstream Service

`UpstreamService` describes the upstream services behind `Rupid`.

### Gateway Constructor

`GatewayConstructor` constructs the `Rupid` from parameters by setting up `Acceptor`, `Router`, `Filters`, `Proxy` and chain them together.

## Data Structure

### RupidContext

```yaml
# to store the context in the whole process chain
```



### Acceptor

```yaml
Acceptor:
	port: 7443
	ssl:
		cert: /path/to/cert
		key: /path/to/key
    route:
    	- route_name_1
    	- route_name_2
```



### Router

```yaml
Router:
	name: router_name
	filters:
		- filter_chain_a
		- filter_chain_b
  	proxy: proxy_name
	service: service_name
```



### Filters

```yaml
Filters:
	name: filter_chain_name
	filter_list:
		- filter_a
		- filter_b
```



### Filter

```yaml
Filter:
	name: filer_name
	predicate: "(TODO)"
```



### Proxy

```yaml
Proxy:
	ssl_settings: "(TODO)"
```



### Upstream Service

```yaml
Service:
	name: service_name
	uri: http://www.whatever.com/dirA/dirB
```


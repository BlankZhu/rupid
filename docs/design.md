# Rupid API Gateway

## Architecture

```
1. Acceptor
2. Router
3. Pipeline
4. Proxy
5. Service
```

### Acceptor

`Acceptor` listens to a port, accepts HTTP request and maps the request target to a `Router`.

Acceptor should be able to handle HTTPS request from client.

### Router

`Router` will check if the HTTP request is valid, create `RupidContext`, call the global `Pipeline` and the custom `Pipeline`.

### Pipeline

`Pipeline` is a series of plugins, which will pre-process the request, or intercept, or post-process the response, which works like a HTTP middleware but with a lot of combinable logics.

### Plugin

`Plugin` is a HTTP middleware, which may provide URL override, authentication, throttle, circuit break or any other abilities.

### Proxy

`Proxy` will finally get the pre-processed request from filter chains, do the proxy jobs to upstream services and return back the response.

`Proxy` should be able to handle HTTPS to the upstream services.

### Service

`Service` describes the upstream services behind `Rupid`.

### Gateway Constructor

`GatewayConstructor` constructs the `Rupid` from parameters by setting up `Acceptor`, `Router`, `Pipeline`, `Plugin`, `Proxy`, `Service `and chain them together.

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



### Pipeline

```yaml
Pipeline:
	name: pipeline_name
	plugin_list:
		- plugin_a
		- plugin_b
```



### Plugin

```yaml
Plugin:
	name: plugin_name
	field: "(TODO)"
```



### Proxy

```yaml
Proxy:
	ssl_settings: "(TODO)"
```



### Service

```yaml
Service:
	name: service_name
	uri: http://www.whatever.com/dirA/dirB
```


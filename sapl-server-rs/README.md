# SAPL Server RS - A SAPL Authorization Server
This server is a Policy Decision Point (PDP) implemented in [Rust](https://www.rust-lang.org). It uses the Streaming Attribute Policy Language (SAPL) and provides authorization services through an HTTP API. SAPL is further explained on the [SAPL home page](https://sapl.io).

> [!NOTE]
> To ensure that the following commands work, please go to the sapl-server-rs directory.

## Run Application

If Rust has not yet been installed, please follow the [installation instructions](../README.md#prerequisites).

> [!TIP]
> To ensure that the application is compiled without errors, please clone the entire repository and not just copy the sapl-server-rs project.

The following command compiles the application if necessary and then executes it.

```
cargo run
```

If the application is running and the configuration is unchanged, the endpoints can be tested using simple `curl` commands. The policies from [sapl-webflux-demo](https://github.com/heutelbeck/sapl-demos/tree/master/sapl-demo-webflux) are already included and can be used directly.

Of course, other policies can be stored, and with the correct connection configuration, the pdp can be used with other SAPL PEPs.

This request will show you the status and version of the pdp instance.
```
curl --insecure --request GET --url https://127.0.0.1:8443/api/pdp/health
```

This will give you a decision with an obligation as response.
```
curl --insecure --request POST \
  --url https://127.0.0.1:8443/api/pdp/decide \
  --header 'content-type: application/json' \
  --data '{
  "subject": "WILLI",
  "action": {
    "http": {
        "contextPath": "/string"
    }
  },
  "resource": "something"
}'
```

This will give you a decision with an obligation and a resource.
```
curl --insecure --request POST \
  --url https://127.0.0.1:8443/api/pdp/decide \
  --header 'content-type: application/json' \
  --data '{
  "subject": {
    "name": "admin"
  },
  "action": {
    "http": {
        "contextPath": "/changedstring",
        "requestedURI": "http://127.0.0.1:8080/changedstring"
    }
  },
  "resource": "something"
}'
```

This will give you a stream of decisions with obligations, changed every 20 seconds. 

```
curl --insecure --request POST \
  --url https://127.0.0.1:8443/api/pdp/decide \
  --header 'content-type: application/json' \
  --data '{
  "subject":"WILLI",
  "action": {
    "java": {
        "name": "getDocuments"
    }
  },
  "resource": "something"
}'
```

This will give you a stream with different kinds of decisions.
```
curl --insecure --request POST \
  --url https://127.0.0.1:8443/api/pdp/decide \
  --header 'content-type: application/json' \
  --data '{
  "subject":"WILLI",
  "action": {
    "java": {
        "name": "getPatients"
    }
  },
  "resource": "something"
}'
```

## Run Tests

The project contains tests for the API endpoints. To run these tests, please use the following command.

```
cargo test
```

## HTTP API
The application provides an HTTP API with the following endpoints.

### Health

* Url: *{serverUrl}*/api/pdp/health
* Method: GET
* Body: empty
* Produces: A valid json information about status and software version

### Decide

* URL: *{serverURL}*/api/pdp/decide
* Method: POST
* Body: A valid JSON authorization subscription
* Produces: A [SSE](https://en.wikipedia.org/wiki/Server-sent_events) stream of authorization decisions

### Decide Once

* URL: *{serverURL}*/api/pdp/decide-once
* Method: POST
* Body: A valid JSON authorization subscription
* Produces: A single authorization decisions

## Configuration

### Rocket.toml
The basic configuration can be done via the Rocket.toml file. For a more detailed description please see the [rocket configuration documentation](https://rocket.rs/guide/v0.5/configuration/).

* **address**: IP address of the server
* **port**: Port of the server
* **workers**: Number of threads to use for executing the requests
* **max_blocking**: Limit on threads to start for blocking tasks
* **keep_alive**: Keep-alive timeout seconds
* **ident**: If and how to identify via the Server header

### pdp.json
SAPL PDP configurations are possible here. The file can be found in `policies/pdp.json`.

The **algorithm** is used to ensure that the PDP knows how to combine multiple results into an overall result. For more information please visit the [SAPL combining algorithm documentation](https://sapl.io/docs/3.0.0-SNAPSHOT/6_5_CombiningAlgorithm/).

* **DENY_UNLESS_PERMIT**: The decision is `DENY` except there is a `PERMIT`.
* **PERMIT_UNLESS_DENY**: The decision is `PERMIT` except there is a `DENY`.
* **ONLY_ONE_APPLICABLE**: A `PERMIT` or `DENY` decision will only be returned if there is exactly one policy set or policy that evaluates to `PERMIT` or `DENY`.
* **DENY_OVERRIDES**: If a `DENY` decision should prevail a `PERMIT` without setting a default decision.
* **PERMIT_OVERRIDES**: If a `PERMIT` decision should prevail a `DENY` without setting a default decision.
* **FIRST_APPLICABLE**: not allowed

### Log Level

The application default log level is **info**. The log level is defined via the environment variable

```
RUST_LOG
```
If a different log level is required, this can be set via the environment variable or inline with the following command

```
RUST_LOG=debug cargo run
```
The following log levels are possible:

* error
* warn
* info
* debug
* trace

### TLS Configuration
The path to certs and key is defined in the [Rocket.toml](Rocket.toml) file. In the tls folder is a self-signed certificate for quick and easy testing. [Rocket](https:://rocket.rs) works with pem file format. You can create your own certificate with the following command. 

### Generate private key and self-signed certificate
      openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/C=US/ST=State/L=City/O=Organization/OU=OrgUnit/CN=localhost"

This creates:

* key.pem - Private key
* cert.pem - Self-signed certificate
* -nodes means no password protection
* -days 365 sets expiration to 1 year

> [!IMPORTANT]
> The Common Name (CN) should match your domain (use localhost for local testing)

If you also need a p12 format from the same certificate, you can generate one with the following command.
### Convert PEM files to P12
      openssl pkcs12 -export -out certificate.p12 -inkey key.pem -in cert.pem -name "Test Certificate"

> [!CAUTION]
> This example configuration is not intended for production. It contains secrets and certificates which are publicly known. Whenever you run this Pdp-Server with this configuration you accept the resulting risks making the API publicly accessible via the provided credentials and that the server and its decisions cannot be properly authenticated by client applications because of the use of a publicly known self-signed TLS certificate.
> 
> * These are test certificates only - browsers will show security warnings
> * For production, use certificates from a trusted Certificate Authority

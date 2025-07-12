# pdp-server

## TLS Configuration
The path to certs and key is defined in the [Rocket.toml](Rocket.toml) file. In the tls folder is a self-signed certificate for quick and easy testing. [Rocket](https:://rocket.rs) works with pem file format. You can create your own certificate with the following command. 

### Generate private key and self-signed certificate
      openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/C=US/ST=State/L=City/O=Organization/OU=OrgUnit/CN=localhost"

This creates:

* key.pem - Private key
* cert.pem - Self-signed certificate
* -nodes means no password protection
* -days 365 sets expiration to 1 year

**The Common Name (CN) should match your domain (use localhost for local testing)**

If you also need a p12 format from the same certificate, you can generate one with the following command.
### Convert PEM files to P12
      openssl pkcs12 -export -out certificate.p12 -inkey key.pem -in cert.pem -name "Test Certificate"

### Note
This example configuration is not intended for production. It contains secrets and certificates which are publicly known. Whenever you run this Pdp-Server with this configuration you accept the resulting risks making the API publicly accessible via the provided credentials and that the server and its decisions cannot be properly authenticated by client applications because of the use of a publicly known self-signed TLS certificate.

* These are test certificates only - browsers will show security warnings
* For production, use certificates from a trusted Certificate Authority

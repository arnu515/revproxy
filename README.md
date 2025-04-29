# HTTP and Socks5 Proxy

This is a simple proxying service written in Rust.

It is powered by [`fast-socks5`](https://lib.rs/fast-socks5) and [`axum`](https://lib.rs/axum).

## What it is

- A SOCKS5 proxy.
- A HTTP proxy (coming soon) with a web frontend.
- Supports authentication via LDAP.

### In the works

- URL whitelisting/blacklisting support
- HTTP(S) proxy
- Web client

## Configuration

The service is configured using environment variables.

| Name | Description | Default |
| ---- | ----------- | ------- |
| `REVPROXY_SOCKS_HOST` | The binding hostname | `127.0.0.1` |
| `REVPROXY_SOCKS_PORT` | The binding port | `1080` |
| `REVPROXY_HTTP_HOST` | The binding hostname | `127.0.0.1` |
| `REVPROXY_HTTP_PORT` | The binding port | `8080` |
| `REVPROXY_HTTPS` | To enable https, set to `1` | _https disabled_ (`0`) |
| `REVPROXY_HTTPS_HOST` | The binding hostname | `127.0.0.1` |
| `REVPROXY_HTTPS_PORT` | The binding port | `4430` |
| `REVPROXY_HTTPS_CERT` | Path to HTTPS cert | _required_ |
| `REVPROXY_HTTPS_CERT_KEY` | Path to HTTPS cert key | _required_ |
| `REVPROXY_PUB_ADDR` | Proxy public IP, will be the reply address | _optional_ |
| `REVPROXY_{HTTP,SOCKS}_AUTH_METHOD` | Authentication method. See [authentication](#authentication) | `userpass` |
| `REVPROXY_{HTTP,SOCKS}_AUTH_USER` | Username for `userpass` auth | `user` |
| `REVPROXY_{HTTP,SOCKS}_AUTH_PASS` | Password for `userpass` auth | `pass` |
| `REVPROXY_SOCKS_TIMEOUT` | Timeout (sec) (setting to 0 does not disable!) | `10` |
| `REVPROXY_SOCKS_ENABLE_UDP` | To enable proxying UDP connections via SOCKS5, set to `1` | _disabled_ (`0`) |
| `REVPROXY_SOCKS_RESOLVE_DNS` | To disolve resolving DNS in SOCKS5, set to `0` | _enabled_ (`1`) |

Notes:

- If HTTPS is enabled, the CERT and CERT_KEY paths must be set.
  - The HTTP bind will redirect requests to the HTTPS server.
- If HTTPS is disabled, HTTP/3 will also be disabled.
- For HTTP/3, the HTTPS_PORT should also be opened via UDP

## Authentication

There are three available modes:

- `no_auth` -- let anyone through.
- `userpass` -- let some user and password through.
- `ldap` -- coming soon

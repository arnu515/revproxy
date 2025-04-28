mod auth;
pub mod config;
pub mod http;

use std::sync::Arc;

use config::{HttpConfig, SocksConfig};
use fast_socks5::server::{Config as Socks5Config, DenyAuthentication, Socks5Socket};
use tokio::{net::TcpListener, task};

pub async fn start_socks_server(cfg: SocksConfig) -> anyhow::Result<()> {
    if cfg.enable_udp && cfg.pub_addr.is_none() {
        panic!("If UDP is enabled, then PUB_ADDR must be set.")
    }

    let mut config = Socks5Config::<DenyAuthentication>::default();
    config.set_dns_resolve(cfg.resolve_dns);
    config.set_request_timeout(cfg.timeout as u64);
    config.set_udp_support(cfg.enable_udp);
    if let config::Auth::NoAuth = cfg.auth {
        config.set_allow_no_auth(true);
    }
    let config = config.with_authentication(auth::Auth(cfg.auth));
    let config = Arc::new(config);

    let listener = TcpListener::bind(&cfg.addr).await?;

    tracing::info!("Started SOCKS server at {}", cfg.addr);

    loop {
        match listener.accept().await {
            Ok((sock, client_addr)) => {
                tracing::trace!("New client: {client_addr}");
                let mut sock = Socks5Socket::new(sock, config.clone());
                if let Some(addr) = cfg.pub_addr {
                    sock.set_reply_ip(addr);
                }
                task::spawn(async move {
                    match sock.upgrade_to_socks5().await {
                        Ok(sock) => {
                            tracing::info!("Socks5 user connected: {:?}", sock.target_addr());
                        }
                        Err(e) => tracing::error!("Socks5 failed connection: {e:?}"),
                    }
                });
            }
            Err(e) => tracing::error!("Could not handle tcp connection: {e:?}"),
        }
    }
}

pub async fn start_http_server(cfg: HttpConfig) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&cfg.addr).await?;
    tracing::info!("Started HTTP server at {}", listener.local_addr()?);
    crate::http::run_app(listener).await;
    Ok(())
}

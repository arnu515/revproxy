use maud::html;
use salvo::{
    conn::rustls::{Keycert, RustlsConfig},
    prelude::*,
};

use crate::config::HttpConfig;

pub async fn start(cfg: HttpConfig) {
    let router = Router::new().get(index);

    if let Some(https) = cfg.https {
        let cert = match std::fs::read(&https.cert_path) {
            Err(e) => {
                tracing::error!("Could not read certificate path `{}`.", https.cert_path);
                Err::<(), _>(e).unwrap();
                unreachable!();
            }
            Ok(x) => x,
        };
        let key = match std::fs::read(&https.key_path) {
            Err(e) => {
                tracing::error!("Could not read certificate key path `{}`.", https.key_path);
                Err::<(), _>(e).unwrap();
                unreachable!();
            }
            Ok(x) => x,
        };

        let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));

        let force_https_service =
            Service::new(router).hoop(ForceHttps::new().https_port(https.port));

        let listener = TcpListener::new((https.host.clone(), https.port))
            .rustls(config.clone())
            .join(TcpListener::new(cfg.addr));
        let acceptor = QuinnListener::new(
            config.build_quinn_config().unwrap(),
            (https.host, https.port),
        )
        .join(listener)
        .bind()
        .await;

        Server::new(acceptor).serve(force_https_service).await;
    } else {
        let listener = TcpListener::new(cfg.addr).bind().await;
        Server::new(listener).serve(router).await;
    }
}

#[handler]
async fn index(res: &mut Response) {
    res.render(html! {
        h1 { "Hello, world!" }
    })
}

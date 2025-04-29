use revproxy::config::Config;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

struct LogConfig {
    stderr: bool,
    /// for stderr only
    pretty: bool,
    /// json output
    file: bool,
    /// level filter for file log
    file_filter: LevelFilter,
}

impl LogConfig {
    fn new() -> Self {
        Self {
            stderr: true,
            pretty: true,
            file: !cfg!(debug_assertions),
            file_filter: LevelFilter::WARN,
        }
    }
}

fn setup_logging(cfg: LogConfig) -> anyhow::Result<()> {
    let mut layers = Vec::new();

    if cfg.file {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
        let file = std::fs::File::create(format!("revproxy-{timestamp}.log"))?;
        layers.push(
            tracing_subscriber::fmt::layer()
                .with_thread_names(true)
                .with_target(true)
                .json()
                .with_writer(file)
                .with_filter(cfg.file_filter)
                .boxed(),
        );
    }

    if cfg.stderr && cfg.pretty {
        layers.push(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_filter(tracing_subscriber::filter::EnvFilter::from_env("RUST_LOG"))
                .boxed(),
        )
    }
    if cfg.stderr && !cfg.pretty {
        layers.push(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(tracing_subscriber::filter::EnvFilter::from_env("RUST_LOG"))
                .boxed(),
        )
    }

    tracing_subscriber::registry().with(layers).init();

    Ok(())
}

#[tokio::main]
async fn main() {
    setup_logging(LogConfig::new()).unwrap();

    let Config { socks, http } = Config::new();

    tokio::task::spawn(async move { revproxy::start_socks_server(socks).await.unwrap() });
    revproxy::start_http_server(http).await.unwrap();
}

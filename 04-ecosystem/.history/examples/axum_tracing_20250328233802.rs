use axum::{routing::get, Router};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::{self, format::FmtSpan}, Layer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let console = fmt::Layer::new()
    .with_span_events(FmtSpan::New | FmtSpan::CLOSE)
    .pretty()
    .with_filter(LevelFilter::INFO);

  tracing_subscriber::registry()
    .with(console)
    .init();

  let app = Router::new().route("/", get(handler));

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    OK(())
}

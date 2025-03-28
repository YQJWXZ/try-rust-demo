use axum::{routing::get, Router};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    OK(())
}

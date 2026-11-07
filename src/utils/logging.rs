use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init() -> WorkerGuard {
    let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(writer)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
        .init();

    guard
}

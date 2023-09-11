pub mod config;

pub use config::LoggerConfig;
use ctor::ctor;
use time::{format_description::well_known::Rfc3339, UtcOffset};
use tracing::Subscriber;
use tracing_subscriber::fmt::{fmt, time::OffsetTime};

#[ctor]
static LOCAL_TIMEZONE_OFFSET: UtcOffset =
    UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);

pub fn new_logger(config: LoggerConfig) -> impl Subscriber + Send + Sync + 'static {
    fmt()
        .json()
        .with_timer(OffsetTime::new(*LOCAL_TIMEZONE_OFFSET, Rfc3339))
        .with_env_filter(config.log_level)
        .with_ansi(false)
        .finish()
}

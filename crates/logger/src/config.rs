use clap::Parser;

#[derive(Parser)]
pub struct LoggerConfig {
    #[clap(long, default_value = "info", env = "MYSNS_LOG_LEVEL")]
    pub log_level: String,
}

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

// initialize logging
pub fn init(log_format: &str) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    match log_format {
        "json" => {
            // JSON format for production
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        _ => {
            // pretty format for development
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer().pretty())
                .init();
        }
    }

    Ok(())
}

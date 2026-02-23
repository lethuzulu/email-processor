use actix_web::{App, HttpServer, middleware, web};
use email_processor::{
    api::{AppState, routes},
    infrastructure::{logging, metrics, Config},
};
use std::net::SocketAddr;
use tracing::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // load environment variables
    dotenv::dotenv().ok();

    // load config
    let config = Config::load().expect("Failed to load configuration.");
    config.validate().expect("Invalid cofniguration");

    // initialize logging
    logging::init(&config.observability.log_format).expect("Failed to initialize logging");

    info!("Starting email processor service");
    info!(
        host = %config.server.host,
        port = config.server.port,
        workers = config.server.workers,
        "Server configuration"
    );

    // initialize metrics on separate port
    let metrics_addr: SocketAddr = "0.0.0.0:9090".parse()?;
    metrics::init_metrics(metrics_addr).map_err(|e| anyhow::anyhow!("{}", e))?;
    info!("Metrics server started on {}", metrics_addr);

    // create shared application state
    let app_state = AppState::new(config.clone());

    // build server
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    info!(bind_addr = %bind_addr, "Binding server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            // middleware
            .wrap(middleware::Logger::default()) // HTTP request logging
            .wrap(middleware::Compress::default()) // Response compression
            // routes
            .configure(routes::configure)
    })
    .workers(config.server.workers)
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}

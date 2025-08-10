use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpServer};
use config::secret_config::SecretConfig;
use error::ApiError;
use gql::schema::{create_schema, AppSchema};
use middleware::{access_token_validator, logging_transaction};
use tokio::select;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};
use tracing::instrument;
use tracing::{error, info};

mod db;
mod error;
mod gql;
mod middleware;
mod routes;

/// Actix Web web server main function.
/// Launch the GraphQL server.
///
/// # Examples
///
/// ```rust
/// fn main() -> Result<()> {
///
/// match api::main() {
/// Ok(_) => println!("API main completed running"),
/// Err(e) => {
/// println!("Error!");
/// std::process::exit(1);
///     }
///         }
/// Ok(())
/// }
/// ```
#[instrument]
#[actix_web::main]
pub async fn main() -> Result<(), ApiError> {
    env_logger::init();
    let schema: AppSchema = create_schema().await?;
    let secret_config = SecretConfig::new()?;
    let server = HttpServer::new(move || {
        App::new()
            .configure(routes::configure_routes)
            .app_data(web::Data::new(secret_config.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(from_fn(access_token_validator))
            .wrap(from_fn(logging_transaction))
    })
    .bind("0.0.0.0:8080")
    .map_err(ApiError::ServerError)?
    .run();

    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
    select! {
        res = server => {
            if let Err(e) = res {
                error!("Server encounterd an error: {}", e);
            }
        }

          _ = signal::ctrl_c() => {
        info!("🛑 SIGINT received (Ctrl+C), shutting down gracefully...");
    }
    _ = sigterm.recv() => {
        info!("🛑 SIGTERM received, shutting down gracefully...");
    }
    }

    info!("Server shutdown completed");
    Ok(())
}

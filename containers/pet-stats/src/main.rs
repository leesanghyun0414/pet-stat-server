use anyhow::Result;
use config::{auth_config::AuthConfig, base_config::Config};
use tracer::init_tracing;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> Result<()> {
    init_tracing()?;
    info!("The Application has been started");
    let a = AuthConfig::new()?;
    info!("{}", a.google_oauth_public_key_url);
    match api::main() {
        Ok(_) => info!("API main completed successfully"),
        Err(e) => {
            error!("API main failed: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

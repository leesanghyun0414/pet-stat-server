use anyhow::Result;
use tracer::init_tracing;
use tracing::{error, info};

#[tracing::instrument]
fn main() -> Result<()> {
    init_tracing()?;
    info!("The Application has been started");
    match api::main() {
        Ok(_) => info!("API main completed successfully"),
        Err(e) => {
            error!("API main failed: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

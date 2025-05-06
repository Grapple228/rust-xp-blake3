// region:    --- Modules

use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

// -- Modules
mod config;
mod error;

// -- Flatten
pub use config::config;
pub use error::{Error, Result};

// endregion: --- Modules

pub fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");

    // CONFIG INITIALIZATION
    info!("Loading config...");
    let config = config();
    debug!("{:?}", config);

    Ok(())
}

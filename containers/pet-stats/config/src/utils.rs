use crate::error::ConfigError;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::{error, info, instrument};

#[instrument]
pub(crate) fn load_config<T>() -> Result<T, ConfigError>
where
    T: DeserializeOwned + Debug,
{
    envy::from_env::<T>()
        .inspect(|_| info!("Successfully loaded application configurations: "))
        .inspect_err(|error| {
            error!(
                "Error occurred getting application configurations: {:?}",
                error
            )
        })
        .map_err(ConfigError::Envy)
}
#[instrument]
pub(crate) fn load_config_with_prefix<T>(prefix: String) -> Result<T, ConfigError>
where
    T: DeserializeOwned + Debug,
{
    envy::prefixed(prefix)
        .from_env::<T>()
        .inspect(|_| info!("Successfully loaded application configurations: "))
        .inspect_err(|error| {
            error!(
                "Error occurred getting application configurations: {:?}",
                error
            )
        })
        .map_err(ConfigError::Envy)
}

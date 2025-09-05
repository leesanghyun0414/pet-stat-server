use crate::error::ConfigError;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::{error, info, instrument};

/// Load the configurations generic from T , Don't have prefix option.
///
/// # Errors
///
/// ConfigError occurred when initilizing envy fron envinronment variables.
///
/// # Examples
///
/// ```
/// load_config::<AuthConfig>().unwrap();
/// ```
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

/// Load the configurations generic from T, Having prefix option.
///
/// # Arguments
///
/// * `prefix` - OS envinronment variable Prefix string.
///
/// # Errors
///
/// ConfigError occurred when initilizing envy fron envinronment variables.
///
///
/// # Examples
///
/// ```
/// load_config_with_prefix::<SomeConfig>("env_".to_owned());
/// ```
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

use crate::error::ConfigError;

pub trait Config: Sized {
    fn new() -> Result<Self, ConfigError>;
}

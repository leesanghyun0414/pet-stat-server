use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUser {
    pub provider_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OAuthProvider {
    Google,
    Apple,
    Meta,
}

# Description

The Pet Stats application server container. 

# Structures

Explain this project directory Structures.

## Workspaces

Cargo Workspaces.

### Entity Library

The Library for mapping from other data source structures.

### Migration Library

The Library for migrating schema to DB.

### API Library

The Library for providing server API endpoint logics.

### Service Library

The Library for providing business logics for API endpoints.

### Tracer Library

The Library for providing logging setting for application.



### Config Library

The Library for providing Configurations from environment variables using envy crate.

#### Examples

```rust

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde_derive::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct DbInfo {
    postgres_user: String,
    postgres_password: String,
    postgres_host: String,
    postgres_db: String,
    postgres_port: String,
}

impl DbInfo {
    pub(crate) fn database_url(&self) -> String {
        format! {
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        }
    }
}

pub async fn database_connection() -> DatabaseConnection {
    let database_url = match envy::from_env::<DbInfo>() {
        Ok(val) => val.database_url(),
        Err(err) => panic!("Can't read envs.\n{}", err),
    };
    let opt = ConnectOptions::new(database_url)
        .max_connections(10)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .to_owned();
    Database::connect(opt)
        .await
        .expect("Database Can't been connected.")
}
```

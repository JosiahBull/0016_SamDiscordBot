use std::{ops::Deref, sync::Arc, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DatabaseHandle {
    pub pool: Arc<DatabaseConnection>,
}

impl DatabaseHandle {
    pub async fn connect(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opt = ConnectOptions::new(url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let conn = Database::connect(opt).await?;

        Ok(DatabaseHandle {
            pool: Arc::new(conn),
        })
    }
}

impl Deref for DatabaseHandle {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

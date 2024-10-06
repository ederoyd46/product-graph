use std::env;

use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

use super::error::UnexpectedError;
use super::ApplicationError;

pub struct ApplicationContextBuilder {
    database_url: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
}

impl ApplicationContextBuilder {
    pub fn new() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or("127.0.0.1:8000".to_string()),
            database_username: env::var("DATABASE_USERNAME").unwrap_or("root".to_string()),
            database_password: env::var("DATABASE_PASSWORD").unwrap_or("root".to_string()),
            database_namespace: env::var("DATABASE_NAMESPACE").unwrap_or("test".to_string()),
            database_name: env::var("DATABASE_NAME").unwrap_or("test".to_string()),
        }
    }

    pub fn with_database_url(mut self, database_url: String) -> Self {
        self.database_url = database_url;
        self
    }

    pub fn with_database_username(mut self, database_username: String) -> Self {
        self.database_username = database_username;
        self
    }

    pub fn with_database_password(mut self, database_password: String) -> Self {
        self.database_password = database_password;
        self
    }

    pub fn with_database_namespace(mut self, database_namespace: String) -> Self {
        self.database_namespace = database_namespace;
        self
    }

    pub fn with_database_name(mut self, database_name: String) -> Self {
        self.database_name = database_name;
        self
    }

    pub async fn build(self) -> Result<ApplicationContext, ApplicationError> {
        let database_context_sdk = DatabaseSdkContext {
            database_url: self.database_url.clone(),
            database_username: self.database_username.clone(),
            database_password: self.database_password.clone(),
            database_namespace: self.database_namespace.clone(),
            database_name: self.database_name.clone(),
            client: self.init_database_connection().await?,
        };

        Ok(ApplicationContext {
            database: database_context_sdk,
        })
    }

    async fn init_database_connection(&self) -> Result<Surreal<Db>, ApplicationError> {
        let db = Surreal::new::<Mem>(()).await.map_err(|e| {
            ApplicationError::Unexpected(UnexpectedError::new(
                "Could not connect to the embedded database".into(),
                e.into(),
            ))
        })?;

        // let db = Surreal::new::<Ws>(&self.database_url).await.map_err(|e| {
        //     ApplicationError::Unexpected(UnexpectedError::new(
        //         "Could not connect to the database".into(),
        //         e.into(),
        //     ))
        // })?;

        // let jwt = db
        //     .signin(Root {
        //         username: &self.database_username,
        //         password: &self.database_password,
        //     })
        //     .await
        //     .map_err(|e| {
        //         ApplicationError::Unexpected(UnexpectedError::new(
        //             "Could not sign in to database".into(),
        //             e.into(),
        //         ))
        //     })?;

        db.use_ns(&self.database_namespace)
            .use_db(&self.database_name)
            .await
            .map_err(|e| {
                ApplicationError::Unexpected(UnexpectedError::new(
                    "Could not use namespace and database".into(),
                    e.into(),
                ))
            })?;

        // let jwt = db
        //     .signin(Database {
        //         username: &self.database_username,
        //         password: &self.database_password,
        //         namespace: &self.database_namespace,
        //         database: &self.database_name,
        //     })
        //     .await
        //     .map_err(|e| {
        //         ApplicationError::Unexpected(UnexpectedError::new(
        //             "Could not sign in to database".into(),
        //             e.into(),
        //         ))
        //     })?;
        //
        // info!("JWT: {:?}", jwt.into_insecure_token().to_string());
        log::info!("Started Database");
        Ok(db)
    }
}

impl Default for ApplicationContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct DatabaseSdkContext {
    database_url: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
    client: Surreal<Db>,
}

impl DatabaseSdkContext {
    pub fn get_database_info(&self) -> String {
        format!(
            "Database: {}://{}:{}@{}/{}",
            self.database_url,
            self.database_username,
            self.database_password,
            self.database_namespace,
            self.database_name
        )
    }

    pub fn get_connection(&self) -> &Surreal<Db> {
        &self.client
    }
}

#[derive(Clone)]
pub struct ApplicationContext {
    pub database: DatabaseSdkContext,
}

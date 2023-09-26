use reqwest::{header, header::HeaderValue, Client as HttpClient, RequestBuilder};

use std::env;

use surrealdb::engine::remote::ws::{Client as SurrealClient, Ws};
use surrealdb::opt::auth::Database;
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
            database_url: env::var("DATABASE_URL").unwrap_or("http://localhost:8000".to_string()),
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

    pub fn build(self) -> ApplicationContext {
        let database_context_rest = DatabaseRestContext {
            database_url: self.database_url.clone(),
            database_username: self.database_username.clone(),
            database_password: self.database_password.clone(),
            database_namespace: self.database_namespace.clone(),
            database_name: self.database_name.clone(),
        };

        let database_context_sdk = DatabaseSdkContext {
            database_url: self.database_url.clone(),
            database_username: self.database_username.clone(),
            database_password: self.database_password.clone(),
            database_namespace: self.database_namespace.clone(),
            database_name: self.database_name.clone(),
        };

        ApplicationContext {
            database_rest: database_context_rest,
            database_sdk: database_context_sdk,
        }
    }
}

impl Default for ApplicationContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DatabaseRestContext {
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub database_namespace: String,
    pub database_name: String,
}

impl DatabaseRestContext {
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
    pub fn reqwest_builder(&self, method: reqwest::Method, extension: &str) -> RequestBuilder {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "NS",
            HeaderValue::from_str(&self.database_namespace).unwrap(),
        );
        headers.insert("DB", HeaderValue::from_str(&self.database_name).unwrap());
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        HttpClient::new()
            .request(method, format!("{}/{}", self.database_url, extension))
            .basic_auth(&self.database_username, Some(&self.database_password))
            .headers(headers)
    }
}

pub struct DatabaseSdkContext {
    database_url: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
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

    pub async fn init_database_connection(
        &self,
    ) -> Result<Surreal<SurrealClient>, ApplicationError> {
        // let db = Surreal::new::<Ws>(&self.database_url).await?;
        let db = Surreal::new::<Ws>("localhost:8000").await.map_err(|e| {
            ApplicationError::Unexpected(UnexpectedError::new(
                "Could not connect to the database".into(),
                e.into(),
            ))
        })?;

        db.signin(Database {
            username: &self.database_username,
            password: &self.database_password,
            namespace: &self.database_namespace,
            database: &self.database_name,
        })
        .await
        .map_err(|e| {
            ApplicationError::Unexpected(UnexpectedError::new(
                "Could not sign in to data".into(),
                e.into(),
            ))
        })?;

        Ok(db)
    }
}

pub struct ApplicationContext {
    pub database_rest: DatabaseRestContext,
    pub database_sdk: DatabaseSdkContext,
}

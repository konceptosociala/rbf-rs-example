use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use surrealdb::engine::remote::ws::*;
use surrealdb::opt::auth::Root;
use surrealdb::opt::IntoEndpoint;
use surrealdb::Surreal;
use surrealdb::sql::Thing;
use pretty_type_name::pretty_type_name;

use crate::error::*;

pub mod cors;

pub trait DatabaseAddress: IntoEndpoint<Ws, Client = Client> + Display {}
impl<T: IntoEndpoint<Ws, Client = Client> + Display> DatabaseAddress for T {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    id: Thing,
}

pub struct Database {
    db: Surreal<Client>
}

impl Database {
    pub async fn new(
        addr: impl DatabaseAddress,
        username: &str,
        password: &str,
    ) -> Result<Database> {
        log::info!("Connecting to database with address `{addr}`");

        let db = Surreal::new::<Ws>(addr).await?;
        let root = Root { username, password };

        log::info!("Logging in with credentials: username = {username}, password = {password}");

        db.signin(root).await
            .map_err(|_| Error::Auth)?;

        db.use_ns("todo").use_db("todo").await?;

        log::info!("Connected successfully!");

        Ok(Database { db })
    }

    pub async fn insert<T: Serialize + 'static>(
        &self, 
        table: &str, 
        record: T,
    ) -> Result<Record> {
        self.db
            .create(table)
            .content(record)
            .await?
            .ok_or(Error::Insertion {
                table: table.to_string(),
                type_name: pretty_type_name::<T>(),
            })
    }

    pub async fn select<T: DeserializeOwned>(&self, table: &str) -> Result<Vec<T>> {
        let result: Vec<T> = self.db.select(table).await?;
        Ok(result)
    }

    pub async fn update<T: Serialize + 'static>(
        &self, 
        table: &str, 
        id: &str, 
        record: T,
    ) -> Result<Option<Record>> {
        let result: Option<Record> = self.db
            .update((table, id))
            .content(record)
            .await?;
        Ok(result)
    }

    pub async fn delete(&self, table: &str, id: &str) -> Result<Option<Record>> {
        let result: Option<Record> = self.db.delete((table, id)).await?;
        Ok(result)
    }
}
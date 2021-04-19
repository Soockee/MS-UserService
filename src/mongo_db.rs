use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};
use futures::StreamExt;
use crate::models::{User,UserRequest};
use crate::Result;
use crate::error::Error::*;

const DB_NAME: &str = "user_db";
const COLL: &str = "users";

const GUID: &str = "_id";
const USERNAME: &str = "username";


#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("user_service".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_users(&self) ->Result<Vec<User>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<User> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_user(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_user(&self, entry: &UserRequest) -> Result<()> {
        let doc = doc! {
            USERNAME: entry.username.clone(),
        };

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
            Ok(())
    }

    pub async fn update_user(&self, id: &str, entry: &UserRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let doc = doc! {
            USERNAME: entry.username.clone(),
        };

        self.get_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_user(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    /**
     * Utilities
     */
    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn doc_to_user(&self, doc: &Document) -> Result<User> {
        let guid = doc.get_object_id(GUID)?;
        let username = doc.get_str(USERNAME)?;
        
        let user = User {
            guid: guid.to_hex(),
            username: username.to_owned()
        };
        Ok(user)
    }
}
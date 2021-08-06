use postgres::{Client, NoTls};
use crate::models::{User,UserRequest};
use crate::Result;

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
		Ok(Self {
			client: Client::connect("postgresql://postgres:free@localhost:5432/postgres", NoTls)?
		})
	}

	pub async fn update_db(&mut self){
		let result = self.client.batch_execute("
        CREATE TABLE IF NOT EXISTS user  (
            id              SERIAL PRIMARY KEY,
            guid           VARCHAR NOT NULL,
            username       INTEGER NOT NULL REFERENCES author
            )")?;
		Ok(result);
	}

	pub async fn create_user(&mut self, entry: &UserRequest){
		let result = self.client.query("INSERT INTO user (guid, username) VALUES ($1, $2)",
						  &[&entry.username, &entry.guuid]);

		Ok(result);
	}

	pub async fn fetch_users(&mut self) {

		let mut result = Vec::new();

		for row in self.client.query("SELECT id, name, country FROM user", &[])? {
			let user = User {
				id: row.get(0),
				guid: row.get(1),
				username: row.get(2)
			};
			result.push(user);
			println!("User {} is from {}", user.name, user.country);
		}

		Ok(result);
	}
}


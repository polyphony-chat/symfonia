// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::ConfigValue;
use serde_json::{Map, Value};
use sqlx::PgPool;
use tokio::io::AsyncReadExt;

use crate::errors::Error;

#[derive(Debug, Clone, Default)]
pub struct Config(chorus::types::ConfigValue);

impl Deref for Config {
	type Target = chorus::types::ConfigValue;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Config {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Config {
	pub async fn init(db: &PgPool) -> Result<Self, Error> {
		let config = if let Ok(confg_path) = std::env::var("CONFIG_PATH") {
			if let Ok(mut f) = tokio::fs::File::open(&confg_path).await {
				let mut data = String::new();
				f.read_to_string(&mut data).await?;

				Self(serde_json::from_str(&data)?)
			} else {
				Config::default()
			}
		} else {
			let pairs = ConfigEntity::collect_entities(db).await?;
			Config::from_pairs(pairs)
		};

		Ok(config)
	}

	fn from_pairs(pairs: Vec<ConfigEntity>) -> Self {
		let mut value = Value::Object(Map::new());

		for p in pairs {
			let keys: Vec<&str> = p.key.split('_').collect();
			let mut path = vec![];

			for (i, &key) in keys.iter().enumerate() {
				path.push(key);

				if i == keys.len() - 1 {
					insert_into(&mut value, &path, p.value.clone().unwrap_or(Value::Null));
				} else if keys[i + 1].parse::<usize>().is_ok() {
					if !path_exists(&value, &path) {
						insert_into(&mut value, &path, Value::Array(Vec::new()));
					}
				} else if !path_exists(&value, &path) {
					insert_into(&mut value, &path, Value::Object(Map::new()));
				}
			}
		}

		// TODO: Remove this eventually
		let s = serde_json::to_string_pretty(&value).unwrap();
		std::fs::write("debug.json", &s);
		let jd = &mut serde_json::Deserializer::from_str(&s);
		let cf: ConfigValue = serde_path_to_error::deserialize(jd).unwrap();
		match serde_json::from_value(value) {
			Ok(v) => Self(v),
			Err(e) => {
				log::error!(target: "symfonia::api::cfg", "Failed to parse config: {}", e);
				Self(ConfigValue::default())
			}
		}
	}

	fn generate_pairs(obj: &Value, key: &str) -> Vec<ConfigEntity> {
		let mut pairs = Vec::new();
		match obj {
			Value::Object(map) => {
				for (k, v) in map {
					let new_key =
						if key.is_empty() { k.to_string() } else { format!("{}_{}", key, k) };
					pairs.extend(Self::generate_pairs(v, &new_key));
				}
			}
			Value::Array(arr) => {
				for (i, v) in arr.iter().enumerate() {
					let new_key = format!("{}_{}", key, i);
					pairs.extend(Self::generate_pairs(v, &new_key));
				}
			}
			_ => pairs.push(ConfigEntity(chorus::types::ConfigEntity {
				key: key.to_string(),
				value: Some(obj.clone()),
			})),
		}
		pairs
	}
}

#[derive(Debug)]
pub struct ConfigEntity(chorus::types::ConfigEntity);

impl Deref for ConfigEntity {
	type Target = chorus::types::ConfigEntity;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for ConfigEntity {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl ConfigEntity {
	pub async fn get_entity_by_key(db: &PgPool, key: &str) -> Result<Self, Error> {
		sqlx::query_as("SELECT * FROM config WHERE key = ?")
			.bind(key)
			.fetch_one(db)
			.await
			.map(Self)
			.map_err(Error::Sqlx)
	}

	pub async fn collect_entities(db: &PgPool) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * FROM config")
			.fetch_all(db)
			.await
			.map(|res| res.into_iter().map(Self).collect())
			.map_err(Error::Sqlx)
	}
}

fn path_exists(value: &Value, path: &[&str]) -> bool {
	let mut current = value;

	for &key in path {
		match current {
			Value::Object(map) => {
				if let Some(v) = map.get(key) {
					current = v;
				} else {
					return false;
				}
			}
			Value::Array(arr) => {
				if let Ok(index) = key.parse::<usize>() {
					if let Some(v) = arr.get(index) {
						current = v;
					} else {
						return false;
					}
				} else {
					return false;
				}
			}
			_ => return false,
		}
	}

	true
}

fn insert_into(value: &mut Value, path: &[&str], new_value: Value) {
	let last_key = path.last().unwrap();
	let parent_path = &path[0..path.len() - 1];

	let mut current = value;

	for &key in parent_path {
		current = match current {
			Value::Object(map) => map.get_mut(key).unwrap(),
			Value::Array(arr) => arr.get_mut(key.parse::<usize>().unwrap()).unwrap(),
			_ => unreachable!(),
		};
	}

	match current {
		Value::Object(map) => {
			map.insert((*last_key).to_string(), new_value);
		}
		Value::Array(arr) => {
			let index = last_key.parse::<usize>().unwrap();
			if index >= arr.len() {
				arr.resize(index + 1, Value::Null);
			}
			arr[index] = new_value;
		}
		_ => unreachable!(),
	};
}

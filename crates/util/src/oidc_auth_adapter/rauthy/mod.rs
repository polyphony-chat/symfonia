use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	Admin,
	User,
	Moderator,
	#[serde(other)]
	Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserValues {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthdate: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub street: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub zip: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Rauthy `User` type.
pub struct User {
	#[serde(skip_serializing_if = "Option::is_none")]
	/// UNIX timestamp in seconds
	pub last_login: Option<i64>,

	#[serde(skip_serializing_if = "Option::is_none")]
	/// UNIX timestamp in seconds
	pub password_expires: Option<i64>,

	#[serde(skip_serializing_if = "Option::is_none")]
	/// UNIX timestamp in seconds
	pub picture_id: Option<String>,

	pub roles: Vec<Role>,

	#[serde(skip_serializing_if = "Option::is_none")]
	/// UNIX timestamp in seconds
	pub user_expires: Option<i64>,

	pub user_values: UserValues,

	#[serde(skip_serializing_if = "Option::is_none")]
	/// UNIX timestamp in seconds
	pub webauthn_user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, zeroize::ZeroizeOnDrop, zeroize::Zeroize)]
/// A Rauthy API key.
pub struct ApiKey {
	pub name: String,
	secret: String,
}

impl ApiKey {
	/// Provides a reference to the API keys' secret value. You are advised not
	/// to store this token in other variables. Using only the reference will
	/// more easily ensure that the [zeroize::Zeroize] and
	/// [zeroize::ZeroizeOnDrop] traits can work as intended, and strike
	/// the secret value from application memory completely, when it is no
	/// longer needed.
	pub fn secret(&self) -> &str {
		&self.secret
	}

	pub fn is_empty(&self) -> bool {
		self.secret.is_empty() || self.name.is_empty()
	}
}

impl std::fmt::Display for ApiKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("{}${}", self.name, self.secret()))
	}
}

use std::str::FromStr;

use openidconnect::core::CoreIdToken;
use reqwest::Method;
use util::{
	configuration::SymfoniaConfiguration,
	entities::Config,
	oidc_auth_adapter::{AdminApi, Ip},
};

static RAUTHY_REGISTER_PATH: &str = "/users";
static RAUTHY_REGISTER_METHOD: Method = Method::POST;
static RAUTHY_EDIT_PATH: &str = "/users/{id}";
static RAUTHY_EDIT_METHOD: Method = Method::PUT;
static RAUTHY_DELETE_PATH: &str = "/users/{id}";
static RAUTHY_DELETE_METHOD: Method = Method::DELETE;

pub struct Rauthy;

impl AdminApi for Rauthy {
	type User = CoreIdToken;

	type Error = util::errors::Error;

	fn register_user(
		register_schema: &chorus::types::RegisterSchema,
		client_ip: &[Ip],
		server_ip: &[Ip],
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send {
		let config = SymfoniaConfiguration::get();
		async move {
			let register_url = config
				.api
				.oidc
				.idp_url()
				.map_err(|e| util::errors::Error::Custom(e.to_string()))?
				.join(RAUTHY_REGISTER_PATH)
				.map_err(|e| util::errors::Error::Custom(e.to_string()))?;
			let client = reqwest::Client::new();
			let mut request = client.request(RAUTHY_REGISTER_METHOD.clone(), register_url);
			todo!()
		}
	}

	fn edit_user(
		modify_schema: &chorus::types::UserModifySchema,
		client_ip: &str,
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send {
		todo!()
	}

	fn delete_user(
		oidc_sub: &str,
		client_ip: &str,
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
		todo!()
	}

	fn user_oid_sub(user: &Self::User) -> String {
		todo!()
	}
}

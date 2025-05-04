use openidconnect::core::CoreIdToken;
use util::oidc_auth_adapter::AdminApi;

pub struct Rauthy;

impl AdminApi for Rauthy {
	type User = CoreIdToken;

	type Error = util::errors::Error;

	fn register_user(
		login_schema: &chorus::types::RegisterSchema,
		client_ip: &str,
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send {
		todo!()
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

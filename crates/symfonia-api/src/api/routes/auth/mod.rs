/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod login;
mod register;

pub use login::*;
use poem::{Route, post};
pub use register::*;
use util::{entities::User, oidc_auth_adapter::AdminApi};

pub fn setup_routes() -> Route {
	Route::new().at("/login", post(login)).at("/register", post(register))
}

pub struct SpacebarLogin;

impl AdminApi for SpacebarLogin {
	type User = User;

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

	fn login_user(
		login_schema: &chorus::types::LoginSchema,
		client_ip: &str,
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send {
		todo!()
	}

	fn user_oid_sub(user: &Self::User) -> String {
		todo!()
	}
}

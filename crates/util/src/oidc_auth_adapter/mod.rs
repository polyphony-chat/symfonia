/*
Create a confidential OIDC client which can create, read, update and delete OIDC users.
This client will serve as a "bridge" for Spacebar API clients, so that they can
authenticate even though they have no clue about OIDC.
*/

use chorus::types::RegisterSchema;

use crate::errors;

/// Represents a set of administration APIs of an OIDC IDP needed to register a
/// new user. There is no proper, standardized administration API for IDPs, and
/// registering a new client programmatically is not a part of OIDC. IDPs like
/// "Rauthy" will provide an admin API to do this.
pub trait AdminApi {
	/// The user type specific to this admin API implementation
	type User;
	type Error: std::error::Error;

	/// Register a new OIDC user using this admin API implementation.
	///
	/// ## Parameters
	///
	/// - `register_schema` [RegisterSchema]: Registration information provided
	///   by the Spacebar client.
	/// - `client_ip` [str]: IP of the client; MUST be forwarded as `X-Real-Ip`
	///   header to make use of security features.
	fn register_user(
		login_schema: RegisterSchema,
		client_ip: &str,
	) -> impl std::future::Future<Output = Result<Self::User, Self::Error>> + Send;

	/// Retrieve the OIDC `sub` attribute of a `Self::User`
	fn user_oid_sub(user: &Self::User) -> String;
}

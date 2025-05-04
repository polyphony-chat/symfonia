use chorus::types::UserModifySchema;
use log::{error, warn};
use reqwest::{Method, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use util::{
	configuration::SymfoniaConfiguration,
	oidc_auth_adapter::{AdminApi, Ip, ensure_proper_client_ips, rauthy::ApiKey},
};

static RAUTHY_REGISTER_PATH: &str = "/users";
static RAUTHY_REGISTER_METHOD: Method = Method::POST;
static RAUTHY_EDIT_PATH: &str = "/users/{id}";
static RAUTHY_EDIT_METHOD: Method = Method::PUT;
static RAUTHY_DELETE_PATH: &str = "/users/{id}";
static RAUTHY_DELETE_METHOD: Method = Method::DELETE;

pub struct Rauthy;

impl AdminApi for Rauthy {
	type Error = util::errors::Error;
	type ApiKey = ApiKey;

	fn register_user(
		token: ApiKey,
		register_schema: &chorus::types::RegisterSchema,
		client_ips: &[Ip],
		server_ips: &[Ip],
		pool: &sqlx::PgPool,
	) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send {
		let config = SymfoniaConfiguration::get();
		async move {
			if token.is_empty() {
				warn!("Could not register Rauthy user: Empty authorization token provided");
				return Err(util::errors::Error::Custom(
					"Empty authorization token provided".into(),
				));
			}
			let email = match &register_schema.email {
				Some(address) => address.clone(),
				None => {
					return Err(util::errors::Error::User(util::errors::UserError::InvalidEmail));
				}
			};
			if server_ips.is_empty() {
				log::error!("The server_ip slice contains no entries!");
				return Err(util::errors::Error::Custom(
					"The server IPs list is empty".to_string(),
				));
			}
			ensure_proper_client_ips(client_ips).map_err(util::errors::Error::Custom)?;
			let register_url = config
				.api
				.oidc
				.idp_url()
				.map_err(|e| util::errors::Error::Custom(e.to_string()))?
				.join(RAUTHY_REGISTER_PATH)
				.map_err(|e| util::errors::Error::Custom(e.to_string()))?;
			let client = reqwest::Client::new();
			let request = client
				.request(RAUTHY_REGISTER_METHOD.clone(), register_url)
				.headers(construct_x_forwarded_for(client_ips, server_ips))
				.body(
					json!({
						"email": email,
						"given_name": &register_schema.username,
						"language": "en",
						"roles": ["symfonia", "spacebar", "trust_0"],
						"user_expires": null
					})
					.to_string(),
				);
			let response = request.send().await?;
			// TODO: Match response status
			match response.status() {
				StatusCode::OK => (),
				StatusCode::FORBIDDEN => {
					error!(
						"Trying to register a user with Rauthy yielded 403 FORBIDDEN. Does the client have administration privileges?"
					);
					return Err(util::errors::Error::Custom(
						"OIDC client lacks privileges needed to register user".into(),
					));
				}
			};
			// TODO: Login user to receive [CoreIdToken], perhaps with proper scopes?

			// TODO: Return [CoreIdToken]

			todo!()
		}
	}

	fn edit_user(
		token: ApiKey,
		oidc_sub: &str,
		modify_schema: &UserModifySchema,
		client_ip: &[Ip],
		server_ip: &[Ip],
		pool: &PgPool,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
		todo!()
	}

	fn delete_user(
		token: ApiKey,
		oidc_sub: &str,
		client_ips: &[Ip],
		server_ips: &[Ip],
		pool: &PgPool,
	) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
		todo!()
	}
}

/// From a set of client and server [Ip]s, construct a [reqwest::HeaderMap]
/// containing a correctly formatted `X-Forwarded-For` header.
pub fn construct_x_forwarded_for(
	client_ips: &[Ip],
	server_ips: &[Ip],
) -> reqwest::header::HeaderMap {
	let all_ips = client_ips
		.iter()
		.chain(server_ips.iter())
		.map(|ip| ip.to_string())
		.collect::<Vec<_>>()
		.join(", ");

	let mut headers = reqwest::header::HeaderMap::new();
	if !all_ips.is_empty() {
		headers.insert(
			reqwest::header::HeaderName::from_static("x-forwarded-for"),
			reqwest::header::HeaderValue::from_str(&all_ips).expect("Invalid IP address format"),
		);
	}
	headers
}

#[allow(clippy::unwrap_used)]
mod tests {
	use log::debug;
	use reqwest::header::HeaderValue;
	use util::init_logger;

	use super::*;

	#[test]
	fn test_empty_ips() {
		let client_ips: Vec<Ip> = vec![];
		let server_ips: Vec<Ip> = vec![];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		// Should result in an empty header map (no X-Forwarded-For header)
		assert_eq!(headers.len(), 0);
	}

	#[test]
	fn test_client_ips_only() {
		let client_ips = vec![Ip::V4("192.168.1.1".to_string()), Ip::V4("10.0.0.1".to_string())];
		let server_ips: Vec<Ip> = vec![];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str("192.168.1.1, 10.0.0.1").unwrap()
		);
	}

	#[test]
	fn test_server_ips_only() {
		let client_ips: Vec<Ip> = vec![];
		let server_ips = vec![Ip::V4("203.0.113.1".to_string()), Ip::V6("2001:db8::1".to_string())];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str("203.0.113.1, 2001:db8::1").unwrap()
		);
	}

	#[test]
	fn test_both_client_and_server_ips() {
		let client_ips = vec![Ip::V4("192.168.1.1".to_string())];
		let server_ips = vec![Ip::V4("203.0.113.1".to_string())];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str("192.168.1.1, 203.0.113.1").unwrap()
		);
	}

	#[test]
	fn test_mixed_ip_versions() {
		init_logger();
		let client_ips =
			vec![Ip::V4("192.168.1.1".to_string()), Ip::V6("2001:db8:1::1".to_string())];
		let server_ips =
			vec![Ip::V4("203.0.113.1".to_string()), Ip::V6("2001:db8:2::1".to_string())];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);
		debug!("{:?}", headers);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str("192.168.1.1, 2001:db8:1::1, 203.0.113.1, 2001:db8:2::1")
				.unwrap()
		);
	}

	#[test]
	fn test_single_ipv6_handling() {
		// IPv6 addresses should be properly included in X-Forwarded-For
		let client_ips = vec![Ip::V6("2001:db8::1".to_string())];
		let server_ips: Vec<Ip> = vec![];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str("2001:db8::1").unwrap()
		);
	}

	#[test]
	fn test_real_world_example() {
		// Example with real-world looking IP addresses
		let client_ips =
			vec![Ip::V4("203.0.113.195".to_string()), Ip::V4("198.51.100.178".to_string())];
		let server_ips =
			vec![Ip::V4("192.0.2.44".to_string()), Ip::V6("2001:db8::8a2e:370:7334".to_string())];

		let headers = construct_x_forwarded_for(&client_ips, &server_ips);

		assert_eq!(headers.len(), 1);
		assert_eq!(
			headers.get("x-forwarded-for").unwrap(),
			&HeaderValue::from_str(
				"203.0.113.195, 198.51.100.178, 192.0.2.44, 2001:db8::8a2e:370:7334"
			)
			.unwrap()
		);
	}
}

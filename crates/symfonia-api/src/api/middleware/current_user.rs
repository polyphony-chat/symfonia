// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::jwt::Claims;
use poem::{Endpoint, Middleware, Request};
use sqlx::PgPool;
use util::entities::User;

pub struct CurrentUserMiddleware;

impl<E: Endpoint> Middleware<E> for CurrentUserMiddleware {
	type Output = CurrentUserMiddlewareImpl<E>;
	fn transform(&self, ep: E) -> Self::Output {
		Self::Output { ep }
	}
}

pub struct CurrentUserMiddlewareImpl<E> {
	ep: E,
}

impl<E: Endpoint> Endpoint for CurrentUserMiddlewareImpl<E> {
	type Output = E::Output;

	async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
		let db = req.data::<PgPool>().expect("Failed to get database connection");
		if let Some(claims) = req.data::<Claims>() {
			if let Some(user) = User::get_by_id(db, claims.id).await? {
				req.set_data(user);
			}
		}
		self.ep.call(req).await
	}
}

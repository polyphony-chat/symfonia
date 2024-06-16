use chorus::types::jwt::Claims;
use poem::{Endpoint, Middleware, Request};
use sqlx::MySqlPool;

use crate::database::entities::User;

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
        let db = req
            .data::<MySqlPool>()
            .expect("Failed to get database connection");
        println!("Trying to get user from claims");
        if let Some(claims) = req.data::<Claims>() {
            println!("{:?}", claims);
            if let Some(user) = User::get_by_id(db, claims.id).await? {
                println!("Got user");
                req.set_data(user);
            }
        }
        self.ep.call(req).await
    }
}

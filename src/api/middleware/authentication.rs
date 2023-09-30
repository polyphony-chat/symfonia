use crate::database::entities::Config;
use crate::util::token::check_token;
use poem::http::StatusCode;
use poem::{async_trait, Endpoint, Middleware, Request};
use sqlx::MySqlPool;

pub struct AuthenticationMiddleware;

impl<E: Endpoint> Middleware<E> for AuthenticationMiddleware {
    type Output = AuthenticationMiddlewareImpl<E>;
    fn transform(&self, ep: E) -> Self::Output {
        Self::Output { ep }
    }
}

pub struct AuthenticationMiddlewareImpl<E> {
    ep: E,
}

#[async_trait]
impl<E: Endpoint> Endpoint for AuthenticationMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let auth = req
            .header("Authorization")
            .ok_or(poem::error::Error::from_status(StatusCode::UNAUTHORIZED))?;

        let db = req.data::<MySqlPool>().unwrap();
        let cfg = req.data::<Config>().unwrap();

        let claims = check_token(
            db,
            auth.trim_start_matches("Bearer "),
            &cfg.security.jwt_secret,
        )
        .await
        .unwrap();
        req.set_data(claims);

        self.ep.call(req).await
    }
}

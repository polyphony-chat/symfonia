use std::env;

use chorus::types::{PingInstance, PingReturn};
use poem::{handler, web::Json, IntoResponse, Route};
use serde::Serialize;

pub fn setup_routes() -> Route {
    Route::new().at("/ping", ping)
}

#[handler]
pub async fn ping() -> poem::Result<impl IntoResponse> {
    let ping_response = PingReturn {
        // TODO: Fill this with configuration values
        ping: "pong!".to_string(),
        instance: PingInstance {
            id: Some(1.into()),
            name: "todo".to_string(),
            description: None,
            image: None,
            correspondence_email: None,
            correspondence_user_id: None,
            front_page: None,
            tos_page: None,
        },
    };
    Ok(Json(ping_response))
}

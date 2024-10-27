use std::env;

use crate::database::entities::Config;
use chorus::types::{PingInstance, PingReturn};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse, Route,
};
use serde::Serialize;

pub fn setup_routes() -> Route {
    Route::new().at("/ping", ping)
}

#[handler]
pub async fn ping(Data(config): Data<&Config>) -> poem::Result<impl IntoResponse> {
    let ping_response = PingReturn {
        ping: "pong!".to_string(),
        instance: PingInstance {
            id: config.general.instance_id,
            name: config.general.instance_name.to_owned(),
            description: config.general.instance_description.to_owned(),
            image: config.general.image.to_owned(),
            correspondence_email: config.general.correspondence_email.to_owned(),
            correspondence_user_id: config.general.correspondence_user_id.to_owned(),
            front_page: config.general.front_page.to_owned(),
            tos_page: config.general.tos_page.to_owned(),
        },
    };
    Ok(Json(ping_response))
}

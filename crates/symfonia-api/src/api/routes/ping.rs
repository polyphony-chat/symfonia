// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::{PingInstance, PingReturn};
use poem::{
	IntoResponse, Route, handler,
	web::{Data, Json},
};
use util::entities::Config;

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

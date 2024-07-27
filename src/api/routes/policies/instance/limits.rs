/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::database::entities::Config;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

#[handler]
pub async fn limits(Data(cfg): Data<&Config>) -> impl IntoResponse {
    Json(serde_json::to_value(&cfg.limits).unwrap())
}

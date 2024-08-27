use std::sync::Arc;

use chorus::types::GatewayResume;
use sqlx::PgPool;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::database::entities::Config;
use crate::errors::Error;

use super::{Connection, NewConnection};

pub(super) async fn resume_connection(
    connection: Arc<Mutex<Connection>>,
    db: PgPool,
    config: Config,
    resume_message: GatewayResume,
) -> Result<NewConnection, Error> {
    // TODO
    todo!()
}

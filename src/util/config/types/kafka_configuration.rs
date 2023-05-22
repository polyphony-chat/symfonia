use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::kafka::KafkaBroker;

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaConfiguration {
    pub brokers: Vec<KafkaBroker>,
}

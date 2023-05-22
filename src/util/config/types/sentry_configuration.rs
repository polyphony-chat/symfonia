use std::ffi::OsString;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryConfiguration {
    pub enabled: bool,
    pub endpoint: String,
    pub trace_sample_rate: f64,
    pub environment: String,
}

impl Default for SentryConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: String::from(
                "https://241c6fb08adb469da1bb82522b25c99f@sentry.quartzinc.space/3",
            ),
            trace_sample_rate: 1.0,
            environment: hostname::get()
                .unwrap_or_else(|_| OsString::new())
                .to_string_lossy()
                .to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::util::config::types::subconfigs::email::{
    mailgun::MailGunConfiguration, mailjet::MailJetConfiguration, sendgrid::SendGridConfiguration,
    smtp::SMTPConfiguration,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum EmailProvider {
    Smtp,
    MailGun,
    MailJet,
    SendGrid,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailConfiguration {
    pub provider: Option<EmailProvider>,
    pub smtp: SMTPConfiguration,
    pub mailgun: MailGunConfiguration,
    pub mailjet: MailJetConfiguration,
    pub sendgrid: SendGridConfiguration,
}

impl Default for EmailConfiguration {
    fn default() -> Self {
        Self {
            provider: None,
            smtp: SMTPConfiguration::default(),
            mailgun: MailGunConfiguration::default(),
            mailjet: MailJetConfiguration::default(),
            sendgrid: SendGridConfiguration::default(),
        }
    }
}

use chorus::types::Snowflake;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct GuildTemplate {
    inner: chorus::types::GuildTemplate,
    pub id: Snowflake,
}

impl Deref for GuildTemplate {
    type Target = chorus::types::GuildTemplate;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GuildTemplate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

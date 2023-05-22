use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageLimits {
    pub max_characters: u32,
    pub max_tts_characters: u32,
    pub max_reactions: u32,
    pub max_attachment_size: u64,
    pub max_bulk_delete: u32,
    pub max_embed_download_size: u64,
}

impl Default for MessageLimits {
    fn default() -> Self {
        Self {
            max_characters: 1048576,
            max_tts_characters: 160,
            max_reactions: 2048,
            max_attachment_size: 1024 * 1024 * 1024,
            max_bulk_delete: 1000,
            max_embed_download_size: 1024 * 1024 * 5,
        }
    }
}

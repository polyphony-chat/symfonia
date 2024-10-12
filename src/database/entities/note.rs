use std::ops::{Deref, DerefMut};

use chorus::types::UserNote;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
/// A note that a user has written about another user. The target user may be the author of the note.
pub struct Note {
    #[sqlx(flatten)]
    inner: UserNote,
}

impl Deref for Note {
    type Target = UserNote;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Note {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Note {
    pub fn into_inner(self) -> UserNote {
        self.inner
    }

    pub fn as_inner(&self) -> &UserNote {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut UserNote {
        &mut self.inner
    }
}

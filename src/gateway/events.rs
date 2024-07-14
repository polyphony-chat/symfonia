// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::{
    ChannelCreate, ChannelDelete, ChannelUpdate, GuildCreate, GuildDelete, GuildUpdate,
    RelationshipAdd, RelationshipRemove, UserUpdate,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Events {
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    RelationshipAdd(RelationshipAdd),
    RelationshipRemove(RelationshipRemove),
    UserUpdate(UserUpdate),
}

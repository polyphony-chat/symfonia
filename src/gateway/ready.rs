use crate::{
    database::entities::{
        Channel, Config, Emoji, Guild, GuildMember, GuildScheduledEvent, Note, Relationship, Role,
        StageInstance, Sticker, User, VoiceState,
    },
    errors::Error,
};
use chorus::{
    types::{
        ClientInfo, ClientStatusObject, GatewayCapabilities, GatewayGuild, GatewayIntents,
        GatewayReady, GuildDataMode, PresenceUpdate, ReadState, RelationshipType, Session,
        Snowflake, UserNote, VersionedReadStateOrEntries,
    },
    UInt8,
};
use serde_json::json;
use sqlx::PgPool;
use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
};

pub async fn create_ready(
    user_id: Snowflake,
    ip: IpAddr,
    config: &Config,
    intents: GatewayIntents,
    capabilities: GatewayCapabilities,
    db: &PgPool,
) -> Result<GatewayReady, Error> {
    let user = User::get_by_id(db, user_id)
        .await?
        .ok_or(Error::Custom(format!(
            "The user specified by user_id '{user_id}' does not exist in the database"
        )))?;

    let mut member_map = vec![];
    let mut presences = HashMap::new();

    let guild_ids = user.get_guild_ids(db).await?;
    let mut guilds = Vec::with_capacity(guild_ids.len());
    for guild_id in guild_ids.iter() {
        let Some(guild) = Guild::get_by_id(db, *guild_id).await? else {
            continue;
        };

        let mut all_members = vec![];

        let mut last_id = None;
        loop {
            let members = GuildMember::get_by_guild_id(db, guild.id, 512, None).await?;
            if members.len() < 512 {
                break;
            }

            last_id = members.last().map(|x| x.user_data.id);
            all_members.extend(members);
        }

        let presences = all_members
            .iter()
            .map(|member| {
                let update = PresenceUpdate {
                    user: member.user_data.to_public_user(),
                    guild_id: Some(guild.id),
                    status: member.user_data.settings.status,
                    activities: vec![],
                    client_status: Default::default(),
                };
                presences.insert((member.user_data.id, update.guild_id), update.clone());
                update
            })
            .collect::<Vec<_>>();

        member_map.push(
            all_members
                .iter()
                .cloned()
                .map(|x| x.into_inner())
                .collect::<Vec<_>>(),
        );

        let user_member = GuildMember::get_by_id(db, user_id, guild.id)
            .await?
            .expect("User is not a member of the guild");

        guilds.push(GatewayGuild {
            joined_at: user_member.joined_at,
            large: guild.member_count.unwrap_or_default() > 1_000,
            unavailable: false,
            geo_restricted: false,
            member_count: guild.member_count.unwrap_or_default() as u64,
            voice_states: VoiceState::get_by_guild(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            members: all_members.into_iter().map(|x| x.into_inner()).collect(),
            channels: Channel::get_by_guild_id(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            threads: vec![],
            presences,
            stage_instances: StageInstance::get_by_guild_id(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            guild_scheduled_events: GuildScheduledEvent::get_by_guild_id(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            data_mode: GuildDataMode::Full,
            stickers: Sticker::get_by_guild(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            roles: Role::get_by_guild(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            emojis: Emoji::get_by_guild(db, guild.id)
                .await?
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
            premium_subscription_count: 0,
            properties: guild.into_inner(),
        });
    }

    let relationships: Vec<_> = Relationship::get_all_by_id(user_id, db)
        .await?
        .into_iter()
        .map(|x| x.into_inner())
        .collect();

    let private_channels = Channel::get_private_of_user(user_id, db)
        .await?
        .into_iter()
        .map(|x| x.into_inner())
        .collect();

    let notes = if !capabilities.contains(GatewayCapabilities::LAZY_USER_NOTES) {
        let notes_vec: Vec<UserNote> = Note::get_by_author_id(user_id, db)
            .await?
            .into_iter()
            .map(|x| x.into_inner())
            .collect();

        let mut notes = HashMap::new();
        for note in notes_vec.into_iter() {
            notes.insert(note.target_id, note.content);
        }
        notes
    } else {
        Default::default()
    };

    let mut deduped_users = HashSet::new();
    for relation in &relationships {
        let Some(user) = User::get_by_id(db, relation.id).await? else {
            continue;
        };

        if !(capabilities.contains(GatewayCapabilities::NO_AFFINE_USER_IDS)
            && relation.relationship_type.eq(&RelationshipType::Implicit))
        {
            presences.insert(
                (user.id, None),
                PresenceUpdate {
                    status: user.settings.status,
                    user: user.to_public_user(),
                    guild_id: None,
                    activities: vec![],
                    client_status: ClientStatusObject::default(),
                },
            );
        }

        deduped_users.insert(user.to_inner());
    }

    // TODO: The session ID needs to be stored in the database and also removed on
    // session disconnect. This is a temporary solution.
    let session_id = Snowflake::generate().to_string();

    // TODO: This is also just temporary.
    let session = Session {
        activities: None,
        client_info: ClientInfo::default(),
        session_id: session_id.clone(),
        status: "Testing symfonia".to_string(),
    };

    // TODO: There are a lot of missing fields here. Ideally, all of the fields should be
    // populated with the correct data.
    let ready = GatewayReady {
        _trace: vec![],
        analytics_token: "".to_string(),
        auth_session_id_hash: "".to_string(),
        country_code: "".to_string(),
        api_version: UInt8::from(9), // This should be dynamic if we decide to support other api versions
        user: user.clone().to_inner(),
        guilds,
        session_id,
        session_type: "normal".to_string(),
        resume_gateway_url: "".to_string(),
        shard: None,
        user_settings: Some(user.settings.into_inner()),
        user_settings_proto: None,
        relationships,
        friend_suggestion_count: Default::default(),
        private_channels,
        notes,
        merged_presences: None,
        merged_members: Some(member_map.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        users: deduped_users.into_iter().collect::<Vec<_>>(),
        auth_token: None,
        authenticator_types: vec![], // Pending MFA implementation
        required_action: None,
        geo_ordered_rtc_regions: vec![],
        tutorial: None,
        api_code_version: Default::default(),
        experiments: vec![],
        sessions: Some([session].into()),
        // Note: Discord.com now just sends Entries, while Spacebar sends VersionedReadState
        read_state: VersionedReadStateOrEntries::Versioned(ReadState {
            entries: Default::default(),
            partial: false,
            version: 0,
        }),
        presences: Some(presences.into_iter().map(|(_, p)| p).collect::<Vec<_>>()),
        guild_experiments: vec![],
    };
    log::debug!(target: "symfonia::gateway::ready::create_ready", "Created READY json payload: {:#?}", json!(ready));
    Ok(ready)
}

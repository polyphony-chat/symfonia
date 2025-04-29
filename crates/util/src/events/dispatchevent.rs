// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use crate::errors::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This enum is supposed to represent all possible dispatch events that can be
/// received from or sent to the gateway. If a variant is missing, it might just
/// be because we haven't caught it yet.
#[serde(rename_all = "PascalCase")]
pub enum DispatchEvent {
	Ready(GatewayPayload<GatewayReady>),
	ReadySupplemental(GatewayPayload<GatewayReadySupplemental>),
	Resumed(GatewayPayload<()>),
	AuthSessionChange(GatewayPayload<()>),
	AuthenticatorCreate(GatewayPayload<()>),
	AuthenticatorUpdate(GatewayPayload<()>),
	AuthenticatorDelete(GatewayPayload<()>),
	ApplicationCommandPermissionsUpdate(GatewayPayload<()>),
	AutoModerationRuleCreate(GatewayPayload<()>),
	AutoModerationRuleUpdate(GatewayPayload<()>),
	AutoModerationRuleDelete(GatewayPayload<()>),
	AutoModerationActionExecution(GatewayPayload<()>),
	AutoModerationMentionRaidDetection(GatewayPayload<()>),
	CallCreate(GatewayPayload<()>),
	CallUpdate(GatewayPayload<()>),
	CallDelete(GatewayPayload<()>),
	ChannelCreate(GatewayPayload<ChannelCreate>),
	ChannelUpdate(GatewayPayload<ChannelUpdate>),
	ChannelDelete(GatewayPayload<ChannelDelete>),
	ChannelStatuses(GatewayPayload<()>),
	VoiceChannelStatusUpdate(GatewayPayload<()>),
	ChannelPinsUpdate(GatewayPayload<()>),
	ChannelRecipientAdd(GatewayPayload<()>),
	ChannelRecipientRemove(GatewayPayload<()>),
	DmSettingsUpsellShow(GatewayPayload<()>),
	ThreadCreate(GatewayPayload<ThreadCreate>),
	ThreadUpdate(GatewayPayload<ThreadUpdate>),
	ThreadDelete(GatewayPayload<ThreadDelete>),
	ThreadListSync(GatewayPayload<ThreadListSync>),
	ThreadMemberUpdate(GatewayPayload<ThreadMemberUpdate>),
	ThreadMembersUpdate(GatewayPayload<ThreadMembersUpdate>),
	FriendSuggestionCreate(GatewayPayload<()>),
	FriendSuggestionDelete(GatewayPayload<()>),
	GuildCreate(GatewayPayload<GuildCreate>),
	GuildUpdate(GatewayPayload<GuildUpdate>),
	GuildDelete(GatewayPayload<GuildDelete>),
	GuildAuditLogEntryCreate(GatewayPayload<()>),
	GuildBanAdd(GatewayPayload<GuildBanAdd>),
	GuildBanRemove(GatewayPayload<GuildBanRemove>),
	GuildEmojisUpdate(GatewayPayload<GuildEmojisUpdate>),
	GuildStickersUpdate(GatewayPayload<()>),
	GuildJoinRequestCreate(GatewayPayload<()>),
	GuildJoinRequestUpdate(GatewayPayload<()>),
	GuildJoinRequestDelete(GatewayPayload<()>),
	GuildMemberAdd(GatewayPayload<GuildMemberAdd>),
	GuildMemberRemove(GatewayPayload<GuildMemberRemove>),
	GuildMemberUpdate(GatewayPayload<GuildMemberUpdate>),
	GuildMembersChunk(GatewayPayload<GuildMembersChunk>),
	GuildMembersRequest(GatewayPayload<GatewayRequestGuildMembers>),
	GuildRoleCreate(GatewayPayload<()>),
	GuildRoleUpdate(GatewayPayload<()>),
	GuildRoleDelete(GatewayPayload<()>),
	GuildScheduledEventCreate(GatewayPayload<()>),
	GuildScheduledEventUpdate(GatewayPayload<()>),
	GuildScheduledEventDelete(GatewayPayload<()>),
	GuildScheduledEventUserAdd(GatewayPayload<()>),
	GuildScheduledEventUserRemove(GatewayPayload<()>),
	GuildSoundboardSoundCreate(GatewayPayload<()>),
	GuildSoundboardSoundUpdate(GatewayPayload<()>),
	GuildSoundboardSoundDelete(GatewayPayload<()>),
	SoundboardSounds(GatewayPayload<()>),
	GuildIntegrationsUpdate(GatewayPayload<GuildIntegrationsUpdate>),
	IntegrationCreate(GatewayPayload<()>),
	IntegrationUpdate(GatewayPayload<()>),
	IntegrationDelete(GatewayPayload<()>),
	InteractionCreate(GatewayPayload<InteractionCreate>),
	InviteCreate(GatewayPayload<InviteCreate>),
	InviteDelete(GatewayPayload<InviteDelete>),
	MessageCreate(GatewayPayload<MessageCreate>),
	MessageUpdate(GatewayPayload<MessageUpdate>),
	MessageDelete(GatewayPayload<MessageDelete>),
	MessageDeleteBulk(GatewayPayload<MessageDeleteBulk>),
	MessagePollVoteAdd(GatewayPayload<()>),
	MessagePollVoteRemove(GatewayPayload<()>),
	MessageReactionAdd(GatewayPayload<MessageReactionAdd>),
	MessageReactionAddMany(GatewayPayload<()>),
	MessageReactionRemove(GatewayPayload<MessageReactionRemove>),
	MessageReactionRemoveAll(GatewayPayload<MessageReactionRemoveAll>),
	MessageReactionRemoveEmoji(GatewayPayload<MessageReactionRemoveEmoji>),
	RecentMentionDelete(GatewayPayload<()>),
	LastMessages(GatewayPayload<()>),
	Oauth2TokenRevoke(GatewayPayload<()>),
	PresenceUpdate(GatewayPayload<PresenceUpdate>),
	RelationshipAdd(GatewayPayload<()>),
	RelationshipUpdate(GatewayPayload<()>),
	RelationshipRemove(GatewayPayload<()>),
	StageInstanceCreate(GatewayPayload<StageInstanceCreate>),
	StageInstanceUpdate(GatewayPayload<StageInstanceUpdate>),
	StageInstanceDelete(GatewayPayload<StageInstanceDelete>),
	TypingStart(GatewayPayload<TypingStartEvent>),
	UserUpdate(GatewayPayload<UserUpdate>),
	UserApplicationRemove(GatewayPayload<()>),
	UserConnectionsUpdate(GatewayPayload<()>),
	UserNoteUpdate(GatewayPayload<()>),
	UserRequiredActionUpdate(GatewayPayload<()>),
	UserSettingsUpdate(GatewayPayload<()>),
	VoiceStateUpdate(GatewayPayload<VoiceStateUpdate>),
	VoiceServerUpdate(GatewayPayload<VoiceServerUpdate>),
	VoiceChannelEffectSend(GatewayPayload<()>),
	WebhooksUpdate(GatewayPayload<WebhooksUpdate>),
}

impl From<DispatchEvent> for Event {
	fn from(value: DispatchEvent) -> Self {
		Self::Dispatch(value)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// This enum is supposed to represent all possible dispatch events that can be
/// received from or sent to the gateway. If a variant is missing, it might just
/// be because we haven't caught it yet.
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEventType {
	Ready,
	ReadySupplemental,
	Resumed,
	AuthSessionChange,
	AuthenticatorCreate,
	AuthenticatorUpdate,
	AuthenticatorDelete,
	ApplicationCommandPermissionsUpdate,
	AutoModerationRuleCreate,
	AutoModerationRuleUpdate,
	AutoModerationRuleDelete,
	AutoModerationActionExecution,
	AutoModerationMentionRaidDetection,
	CallCreate,
	CallUpdate,
	CallDelete,
	ChannelCreate,
	ChannelUpdate,
	ChannelDelete,
	ChannelStatuses,
	VoiceChannelStatusUpdate,
	ChannelPinsUpdate,
	ChannelRecipientAdd,
	ChannelRecipientRemove,
	DmSettingsUpsellShow,
	ThreadCreate,
	ThreadUpdate,
	ThreadDelete,
	ThreadListSync,
	ThreadMemberUpdate,
	ThreadMembersUpdate,
	FriendSuggestionCreate,
	FriendSuggestionDelete,
	GuildCreate,
	GuildUpdate,
	GuildDelete,
	GuildAuditLogEntryCreate,
	GuildBanAdd,
	GuildBanRemove,
	GuildEmojisUpdate,
	GuildStickersUpdate,
	GuildJoinRequestCreate,
	GuildJoinRequestUpdate,
	GuildJoinRequestDelete,
	GuildMemberAdd,
	GuildMemberRemove,
	GuildMemberUpdate,
	GuildMembersChunk,
	GuildRoleCreate,
	GuildRoleUpdate,
	GuildRoleDelete,
	GuildScheduledEventCreate,
	GuildScheduledEventUpdate,
	GuildScheduledEventDelete,
	GuildScheduledEventUserAdd,
	GuildScheduledEventUserRemove,
	GuildSoundboardSoundCreate,
	GuildSoundboardSoundUpdate,
	GuildSoundboardSoundDelete,
	SoundboardSounds,
	GuildIntegrationsUpdate,
	IntegrationCreate,
	IntegrationUpdate,
	IntegrationDelete,
	InteractionCreate,
	InviteCreate,
	InviteDelete,
	MessageCreate,
	MessageUpdate,
	MessageDelete,
	MessageDeleteBulk,
	MessagePollVoteAdd,
	MessagePollVoteRemove,
	MessageReactionAdd,
	MessageReactionAddMany,
	MessageReactionRemove,
	MessageReactionRemoveAll,
	MessageReactionRemoveEmoji,
	RecentMentionDelete,
	LastMessages,
	Oauth2TokenRevoke,
	PresenceUpdate,
	RelationshipAdd,
	RelationshipUpdate,
	RelationshipRemove,
	StageInstanceCreate,
	StageInstanceUpdate,
	StageInstanceDelete,
	TypingStart,
	UserUpdate,
	UserApplicationRemove,
	UserConnectionsUpdate,
	UserNoteUpdate,
	UserRequiredActionUpdate,
	UserSettingsUpdate,
	VoiceStateUpdate,
	VoiceServerUpdate,
	VoiceChannelEffectSend,
	WebhooksUpdate,
}

impl std::fmt::Display for DispatchEventType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// use serde to serialize the enum variant to a SCRERAMING_SNAKE_CASE string
		let serialized = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;

		// strip quotes generated by serde_json
		let trimmed = serialized.trim_matches('"');

		write!(f, "{}", trimmed)
	}
}

impl TryFrom<&str> for DispatchEventType {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		// (ab)use serde_json to deserialize a Self variant from a SCREAMING_SNAKE_CASE
		// string we have to wrap the value in quotes to make it a valid JSON string.
		// we also call .trim() and .to_uppercase() to give a little more leeway to
		// the caller
		serde_json::from_str::<Self>(&format!("\"{}\"", value.trim().to_uppercase()))
			.map_err(|e| Error::Custom(e.to_string()))
	}
}

impl TryFrom<String> for DispatchEventType {
	type Error = Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		DispatchEventType::try_from(value.as_str())
	}
}

#[cfg(test)]
mod dispatch_event_type_tests {
	// One could stop and think: "Is this really necessary"? And my answer to that
	// is: Would you rather come to terms with one file being longer than you'd
	// like, or would you rather have weird deserialization errors and
	// incompatibilities caused by a faulty Display implementation which you have
	// to debug months later?
	//
	// Thought so. -bitfl0wer
	//
	// P.S.: Testing all enum variants has resulted in me finding three bugs in the
	// Display impl.
	use super::*;

	#[test]
	fn test_ready() {
		let event = DispatchEventType::Ready;
		assert_eq!(event.to_string(), "READY");
		// add capitalization randomization and some whitespace at the end
		assert_eq!(DispatchEventType::try_from("readY ".to_string()).unwrap(), event);
		assert_eq!(DispatchEventType::try_from("READY".to_string()).unwrap(), event);
	}

	#[test]
	fn test_ready_supplemental() {
		let event = DispatchEventType::ReadySupplemental;
		assert_eq!(event.to_string(), "READY_SUPPLEMENTAL");
		assert_eq!(DispatchEventType::try_from("READY_SUPPLEMENTAL".to_string()).unwrap(), event);
	}

	#[test]
	fn test_resumed() {
		let event = DispatchEventType::Resumed;
		assert_eq!(event.to_string(), "RESUMED");
		assert_eq!(DispatchEventType::try_from("RESUMED".to_string()).unwrap(), event);
	}

	#[test]
	fn test_auth_session_change() {
		let event = DispatchEventType::AuthSessionChange;
		assert_eq!(event.to_string(), "AUTH_SESSION_CHANGE");
		assert_eq!(DispatchEventType::try_from("AUTH_SESSION_CHANGE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_authenticator_create() {
		let event = DispatchEventType::AuthenticatorCreate;
		assert_eq!(event.to_string(), "AUTHENTICATOR_CREATE");
		assert_eq!(DispatchEventType::try_from("AUTHENTICATOR_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_authenticator_update() {
		let event = DispatchEventType::AuthenticatorUpdate;
		assert_eq!(event.to_string(), "AUTHENTICATOR_UPDATE");
		assert_eq!(DispatchEventType::try_from("AUTHENTICATOR_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_authenticator_delete() {
		let event = DispatchEventType::AuthenticatorDelete;
		assert_eq!(event.to_string(), "AUTHENTICATOR_DELETE");
		assert_eq!(DispatchEventType::try_from("AUTHENTICATOR_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_application_command_permissions_update() {
		let event = DispatchEventType::ApplicationCommandPermissionsUpdate;
		assert_eq!(event.to_string(), "APPLICATION_COMMAND_PERMISSIONS_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("APPLICATION_COMMAND_PERMISSIONS_UPDATE".to_string())
				.unwrap(),
			event
		);
	}

	#[test]
	fn test_auto_moderation_rule_create() {
		let event = DispatchEventType::AutoModerationRuleCreate;
		assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_CREATE");
		assert_eq!(
			DispatchEventType::try_from("AUTO_MODERATION_RULE_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_auto_moderation_rule_update() {
		let event = DispatchEventType::AutoModerationRuleUpdate;
		assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("AUTO_MODERATION_RULE_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_auto_moderation_rule_delete() {
		let event = DispatchEventType::AutoModerationRuleDelete;
		assert_eq!(event.to_string(), "AUTO_MODERATION_RULE_DELETE");
		assert_eq!(
			DispatchEventType::try_from("AUTO_MODERATION_RULE_DELETE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_auto_moderation_action_execution() {
		let event = DispatchEventType::AutoModerationActionExecution;
		assert_eq!(event.to_string(), "AUTO_MODERATION_ACTION_EXECUTION");
		assert_eq!(
			DispatchEventType::try_from("AUTO_MODERATION_ACTION_EXECUTION".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_auto_moderation_mention_raid_detection() {
		let event = DispatchEventType::AutoModerationMentionRaidDetection;
		assert_eq!(event.to_string(), "AUTO_MODERATION_MENTION_RAID_DETECTION");
		assert_eq!(
			DispatchEventType::try_from("AUTO_MODERATION_MENTION_RAID_DETECTION".to_string())
				.unwrap(),
			event
		);
	}

	#[test]
	fn test_call_create() {
		let event = DispatchEventType::CallCreate;
		assert_eq!(event.to_string(), "CALL_CREATE");
		assert_eq!(DispatchEventType::try_from("CALL_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_call_update() {
		let event = DispatchEventType::CallUpdate;
		assert_eq!(event.to_string(), "CALL_UPDATE");
		assert_eq!(DispatchEventType::try_from("CALL_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_call_delete() {
		let event = DispatchEventType::CallDelete;
		assert_eq!(event.to_string(), "CALL_DELETE");
		assert_eq!(DispatchEventType::try_from("CALL_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_channel_create() {
		let event = DispatchEventType::ChannelCreate;
		assert_eq!(event.to_string(), "CHANNEL_CREATE");
		assert_eq!(DispatchEventType::try_from("CHANNEL_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_channel_update() {
		let event = DispatchEventType::ChannelUpdate;
		assert_eq!(event.to_string(), "CHANNEL_UPDATE");
		assert_eq!(DispatchEventType::try_from("CHANNEL_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_channel_delete() {
		let event = DispatchEventType::ChannelDelete;
		assert_eq!(event.to_string(), "CHANNEL_DELETE");
		assert_eq!(DispatchEventType::try_from("CHANNEL_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_channel_statuses() {
		let event = DispatchEventType::ChannelStatuses;
		assert_eq!(event.to_string(), "CHANNEL_STATUSES");
		assert_eq!(DispatchEventType::try_from("CHANNEL_STATUSES".to_string()).unwrap(), event);
	}

	#[test]
	fn test_voice_channel_status_update() {
		let event = DispatchEventType::VoiceChannelStatusUpdate;
		assert_eq!(event.to_string(), "VOICE_CHANNEL_STATUS_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("VOICE_CHANNEL_STATUS_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_channel_pins_update() {
		let event = DispatchEventType::ChannelPinsUpdate;
		assert_eq!(event.to_string(), "CHANNEL_PINS_UPDATE");
		assert_eq!(DispatchEventType::try_from("CHANNEL_PINS_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_channel_recipient_add() {
		let event = DispatchEventType::ChannelRecipientAdd;
		assert_eq!(event.to_string(), "CHANNEL_RECIPIENT_ADD");
		assert_eq!(
			DispatchEventType::try_from("CHANNEL_RECIPIENT_ADD".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_channel_recipient_remove() {
		let event = DispatchEventType::ChannelRecipientRemove;
		assert_eq!(event.to_string(), "CHANNEL_RECIPIENT_REMOVE");
		assert_eq!(
			DispatchEventType::try_from("CHANNEL_RECIPIENT_REMOVE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_dm_settings_upsell_show() {
		let event = DispatchEventType::DmSettingsUpsellShow;
		assert_eq!(event.to_string(), "DM_SETTINGS_UPSELL_SHOW");
		assert_eq!(
			DispatchEventType::try_from("DM_SETTINGS_UPSELL_SHOW".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_thread_create() {
		let event = DispatchEventType::ThreadCreate;
		assert_eq!(event.to_string(), "THREAD_CREATE");
		assert_eq!(DispatchEventType::try_from("THREAD_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_thread_update() {
		let event = DispatchEventType::ThreadUpdate;
		assert_eq!(event.to_string(), "THREAD_UPDATE");
		assert_eq!(DispatchEventType::try_from("THREAD_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_thread_delete() {
		let event = DispatchEventType::ThreadDelete;
		assert_eq!(event.to_string(), "THREAD_DELETE");
		assert_eq!(DispatchEventType::try_from("THREAD_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_thread_list_sync() {
		let event = DispatchEventType::ThreadListSync;
		assert_eq!(event.to_string(), "THREAD_LIST_SYNC");
		assert_eq!(DispatchEventType::try_from("THREAD_LIST_SYNC".to_string()).unwrap(), event);
	}

	#[test]
	fn test_thread_member_update() {
		let event = DispatchEventType::ThreadMemberUpdate;
		assert_eq!(event.to_string(), "THREAD_MEMBER_UPDATE");
		assert_eq!(DispatchEventType::try_from("THREAD_MEMBER_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_thread_members_update() {
		let event = DispatchEventType::ThreadMembersUpdate;
		assert_eq!(event.to_string(), "THREAD_MEMBERS_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("THREAD_MEMBERS_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_friend_suggestion_create() {
		let event = DispatchEventType::FriendSuggestionCreate;
		assert_eq!(event.to_string(), "FRIEND_SUGGESTION_CREATE");
		assert_eq!(
			DispatchEventType::try_from("FRIEND_SUGGESTION_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_friend_suggestion_delete() {
		let event = DispatchEventType::FriendSuggestionDelete;
		assert_eq!(event.to_string(), "FRIEND_SUGGESTION_DELETE");
		assert_eq!(
			DispatchEventType::try_from("FRIEND_SUGGESTION_DELETE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_create() {
		let event = DispatchEventType::GuildCreate;
		assert_eq!(event.to_string(), "GUILD_CREATE");
		assert_eq!(DispatchEventType::try_from("GUILD_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_update() {
		let event = DispatchEventType::GuildUpdate;
		assert_eq!(event.to_string(), "GUILD_UPDATE");
		assert_eq!(DispatchEventType::try_from("GUILD_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_delete() {
		let event = DispatchEventType::GuildDelete;
		assert_eq!(event.to_string(), "GUILD_DELETE");
		assert_eq!(DispatchEventType::try_from("GUILD_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_audit_log_entry_create() {
		let event = DispatchEventType::GuildAuditLogEntryCreate;
		assert_eq!(event.to_string(), "GUILD_AUDIT_LOG_ENTRY_CREATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_AUDIT_LOG_ENTRY_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_ban_add() {
		let event = DispatchEventType::GuildBanAdd;
		assert_eq!(event.to_string(), "GUILD_BAN_ADD");
		assert_eq!(DispatchEventType::try_from("GUILD_BAN_ADD".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_ban_remove() {
		let event = DispatchEventType::GuildBanRemove;
		assert_eq!(event.to_string(), "GUILD_BAN_REMOVE");
		assert_eq!(DispatchEventType::try_from("GUILD_BAN_REMOVE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_emojis_update() {
		let event = DispatchEventType::GuildEmojisUpdate;
		assert_eq!(event.to_string(), "GUILD_EMOJIS_UPDATE");
		assert_eq!(DispatchEventType::try_from("GUILD_EMOJIS_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_stickers_update() {
		let event = DispatchEventType::GuildStickersUpdate;
		assert_eq!(event.to_string(), "GUILD_STICKERS_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_STICKERS_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_join_request_create() {
		let event = DispatchEventType::GuildJoinRequestCreate;
		assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_CREATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_JOIN_REQUEST_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_join_request_update() {
		let event = DispatchEventType::GuildJoinRequestUpdate;
		assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_JOIN_REQUEST_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_join_request_delete() {
		let event = DispatchEventType::GuildJoinRequestDelete;
		assert_eq!(event.to_string(), "GUILD_JOIN_REQUEST_DELETE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_JOIN_REQUEST_DELETE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_member_add() {
		let event = DispatchEventType::GuildMemberAdd;
		assert_eq!(event.to_string(), "GUILD_MEMBER_ADD");
		assert_eq!(DispatchEventType::try_from("GUILD_MEMBER_ADD".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_member_remove() {
		let event = DispatchEventType::GuildMemberRemove;
		assert_eq!(event.to_string(), "GUILD_MEMBER_REMOVE");
		assert_eq!(DispatchEventType::try_from("GUILD_MEMBER_REMOVE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_member_update() {
		let event = DispatchEventType::GuildMemberUpdate;
		assert_eq!(event.to_string(), "GUILD_MEMBER_UPDATE");
		assert_eq!(DispatchEventType::try_from("GUILD_MEMBER_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_members_chunk() {
		let event = DispatchEventType::GuildMembersChunk;
		assert_eq!(event.to_string(), "GUILD_MEMBERS_CHUNK");
		assert_eq!(DispatchEventType::try_from("GUILD_MEMBERS_CHUNK".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_role_create() {
		let event = DispatchEventType::GuildRoleCreate;
		assert_eq!(event.to_string(), "GUILD_ROLE_CREATE");
		assert_eq!(DispatchEventType::try_from("GUILD_ROLE_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_role_update() {
		let event = DispatchEventType::GuildRoleUpdate;
		assert_eq!(event.to_string(), "GUILD_ROLE_UPDATE");
		assert_eq!(DispatchEventType::try_from("GUILD_ROLE_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_role_delete() {
		let event = DispatchEventType::GuildRoleDelete;
		assert_eq!(event.to_string(), "GUILD_ROLE_DELETE");
		assert_eq!(DispatchEventType::try_from("GUILD_ROLE_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_scheduled_event_create() {
		let event = DispatchEventType::GuildScheduledEventCreate;
		assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_CREATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_scheduled_event_update() {
		let event = DispatchEventType::GuildScheduledEventUpdate;
		assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_scheduled_event_delete() {
		let event = DispatchEventType::GuildScheduledEventDelete;
		assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_DELETE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_DELETE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_scheduled_event_user_add() {
		let event = DispatchEventType::GuildScheduledEventUserAdd;
		assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_USER_ADD");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_USER_ADD".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_scheduled_event_user_remove() {
		let event = DispatchEventType::GuildScheduledEventUserRemove;
		assert_eq!(event.to_string(), "GUILD_SCHEDULED_EVENT_USER_REMOVE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SCHEDULED_EVENT_USER_REMOVE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_soundboard_sound_create() {
		let event = DispatchEventType::GuildSoundboardSoundCreate;
		assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_CREATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_CREATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_soundboard_sound_update() {
		let event = DispatchEventType::GuildSoundboardSoundUpdate;
		assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_guild_soundboard_sound_delete() {
		let event = DispatchEventType::GuildSoundboardSoundDelete;
		assert_eq!(event.to_string(), "GUILD_SOUNDBOARD_SOUND_DELETE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_SOUNDBOARD_SOUND_DELETE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_soundboard_sounds() {
		let event = DispatchEventType::SoundboardSounds;
		assert_eq!(event.to_string(), "SOUNDBOARD_SOUNDS");
		assert_eq!(DispatchEventType::try_from("SOUNDBOARD_SOUNDS".to_string()).unwrap(), event);
	}

	#[test]
	fn test_guild_integrations_update() {
		let event = DispatchEventType::GuildIntegrationsUpdate;
		assert_eq!(event.to_string(), "GUILD_INTEGRATIONS_UPDATE");
		assert_eq!(
			DispatchEventType::try_from("GUILD_INTEGRATIONS_UPDATE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_integration_create() {
		let event = DispatchEventType::IntegrationCreate;
		assert_eq!(event.to_string(), "INTEGRATION_CREATE");
		assert_eq!(DispatchEventType::try_from("INTEGRATION_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_integration_update() {
		let event = DispatchEventType::IntegrationUpdate;
		assert_eq!(event.to_string(), "INTEGRATION_UPDATE");
		assert_eq!(DispatchEventType::try_from("INTEGRATION_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_integration_delete() {
		let event = DispatchEventType::IntegrationDelete;
		assert_eq!(event.to_string(), "INTEGRATION_DELETE");
		assert_eq!(DispatchEventType::try_from("INTEGRATION_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_interaction_create() {
		let event = DispatchEventType::InteractionCreate;
		assert_eq!(event.to_string(), "INTERACTION_CREATE");
		assert_eq!(DispatchEventType::try_from("INTERACTION_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_invite_create() {
		let event = DispatchEventType::InviteCreate;
		assert_eq!(event.to_string(), "INVITE_CREATE");
		assert_eq!(DispatchEventType::try_from("INVITE_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_invite_delete() {
		let event = DispatchEventType::InviteDelete;
		assert_eq!(event.to_string(), "INVITE_DELETE");
		assert_eq!(DispatchEventType::try_from("INVITE_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_create() {
		let event = DispatchEventType::MessageCreate;
		assert_eq!(event.to_string(), "MESSAGE_CREATE");
		assert_eq!(DispatchEventType::try_from("MESSAGE_CREATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_update() {
		let event = DispatchEventType::MessageUpdate;
		assert_eq!(event.to_string(), "MESSAGE_UPDATE");
		assert_eq!(DispatchEventType::try_from("MESSAGE_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_delete() {
		let event = DispatchEventType::MessageDelete;
		assert_eq!(event.to_string(), "MESSAGE_DELETE");
		assert_eq!(DispatchEventType::try_from("MESSAGE_DELETE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_delete_bulk() {
		let event = DispatchEventType::MessageDeleteBulk;
		assert_eq!(event.to_string(), "MESSAGE_DELETE_BULK");
		assert_eq!(DispatchEventType::try_from("MESSAGE_DELETE_BULK".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_reaction_add() {
		let event = DispatchEventType::MessageReactionAdd;
		assert_eq!(event.to_string(), "MESSAGE_REACTION_ADD");
		assert_eq!(DispatchEventType::try_from("MESSAGE_REACTION_ADD".to_string()).unwrap(), event);
	}

	#[test]
	fn test_message_reaction_remove() {
		let event = DispatchEventType::MessageReactionRemove;
		assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE");
		assert_eq!(
			DispatchEventType::try_from("MESSAGE_REACTION_REMOVE".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_message_reaction_remove_all() {
		let event = DispatchEventType::MessageReactionRemoveAll;
		assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE_ALL");
		assert_eq!(
			DispatchEventType::try_from("MESSAGE_REACTION_REMOVE_ALL".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_message_reaction_remove_emoji() {
		let event = DispatchEventType::MessageReactionRemoveEmoji;
		assert_eq!(event.to_string(), "MESSAGE_REACTION_REMOVE_EMOJI");
		assert_eq!(
			DispatchEventType::try_from("MESSAGE_REACTION_REMOVE_EMOJI".to_string()).unwrap(),
			event
		);
	}

	#[test]
	fn test_presence_update() {
		let event = DispatchEventType::PresenceUpdate;
		assert_eq!(event.to_string(), "PRESENCE_UPDATE");
		assert_eq!(DispatchEventType::try_from("PRESENCE_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_typing_start() {
		let event = DispatchEventType::TypingStart;
		assert_eq!(event.to_string(), "TYPING_START");
		assert_eq!(DispatchEventType::try_from("TYPING_START".to_string()).unwrap(), event);
	}

	#[test]
	fn test_user_update() {
		let event = DispatchEventType::UserUpdate;
		assert_eq!(event.to_string(), "USER_UPDATE");
		assert_eq!(DispatchEventType::try_from("USER_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_voice_state_update() {
		let event = DispatchEventType::VoiceStateUpdate;
		assert_eq!(event.to_string(), "VOICE_STATE_UPDATE");
		assert_eq!(DispatchEventType::try_from("VOICE_STATE_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_voice_server_update() {
		let event = DispatchEventType::VoiceServerUpdate;
		assert_eq!(event.to_string(), "VOICE_SERVER_UPDATE");
		assert_eq!(DispatchEventType::try_from("VOICE_SERVER_UPDATE".to_string()).unwrap(), event);
	}

	#[test]
	fn test_webhooks_update() {
		let event = DispatchEventType::WebhooksUpdate;
		assert_eq!(event.to_string(), "WEBHOOKS_UPDATE");
		assert_eq!(DispatchEventType::try_from("WEBHOOKS_UPDATE".to_string()).unwrap(), event);
	}
}

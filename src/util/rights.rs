use bitflags::bitflags;

bitflags! {
    pub struct Rights: u64 {
        const OPERATOR = 1 << 0;
        const MANAGE_APPLICATIONS = 1 << 1;
        const MANAGE_GUILDS = 1 << 2;
        const MANAGE_MESSAGES = 1 << 3;
        const MANAGE_RATE_LIMITS = 1 << 4;
        const MANAGE_ROUTING = 1 << 5;
        const MANAGE_TICKETS = 1 << 6;
        const MANAGE_USERS = 1 << 7;
        const ADD_MEMBERS = 1 << 8;
        const BYPASS_RATE_LIMITS = 1 << 9;
        const CREATE_APPLICATIONS = 1 << 10;
        const CREATE_CHANNELS = 1 << 11;
        const CREATE_DMS = 1 << 12;
        const CREATE_DM_GROUPS = 1 << 13;
        const CREATE_GUILDS = 1 << 14;
        const CREATE_INVITES = 1 << 15;
        const CREATE_ROLES = 1 << 16;
        const CREATE_TEMPLATES = 1 << 17;
        const CREATE_WEBHOOKS = 1 << 18;
        const JOIN_GUILDS = 1 << 19;
        const PIN_MESSAGES = 1 << 20;
        const SELF_ADD_REACTIONS = 1 << 21;
        const SELF_DELETE_MESSAGES = 1 << 22;
        const SELF_EDIT_MESSAGES = 1 << 23;
        const SELF_EDIT_NAME = 1 << 24;
        const SEND_MESSAGES = 1 << 25;
        const USE_ACTIVITIES = 1 << 26;
        const USE_VIDEO = 1 << 27;
        const USE_VOICE = 1 << 28;
        const INVITE_USERS = 1 << 29;
        const SELF_DELETE_DISABLE = 1 << 30;
        const DEBTABLE = 1 << 31;
        const CREDITABLE = 1 << 32;
        const KICK_BAN_MEMBERS = 1 << 33;
        const SELF_LEAVE_GROUPS = 1 << 34;
        const PRESENCE = 1 << 35;
        const SELF_ADD_DISCOVERABLE = 1 << 36;
        const MANAGE_GUILD_DIRECTORY = 1 << 37;
        const POGGERS = 1 << 38;
        const USE_ACHIEVEMENTS = 1 << 39;
        const INITIATE_INTERACTIONS = 1 << 40;
        const RESPOND_TO_INTERACTIONS = 1 << 41;
        const SEND_BACKDATED_EVENTS = 1 << 42;
        const USE_MASS_INVITES = 1 << 43;
        const ACCEPT_INVITES = 1 << 44;
        const SELF_EDIT_FLAGS = 1 << 45;
        const EDIT_FLAGS = 1 << 46;
        const MANAGE_GROUPS = 1 << 47;
        const VIEW_SERVER_STATS = 1 << 48;
        const RESEND_VERIFICATION_EMAIL = 1 << 49;
    }
}

impl Rights {
    pub fn any(&self, permission: Rights, check_operator: bool) -> bool {
        (check_operator && self.contains(Rights::OPERATOR)) || self.contains(permission)
    }

    pub fn has(&self, permission: Rights, check_operator: bool) -> bool {
        (check_operator && self.contains(Rights::OPERATOR)) || self.contains(permission)
    }

    pub fn has_throw(&self, permission: Rights) -> Result<bool, &'static str> {
        if self.has(permission, true) {
            Ok(true)
        } else {
            Err("You are missing the following rights")
        }
    }
}

fn all_rights() -> Rights {
    Rights::OPERATOR
        | Rights::MANAGE_APPLICATIONS
        | Rights::MANAGE_GUILDS
        | Rights::MANAGE_MESSAGES
        | Rights::MANAGE_RATE_LIMITS
        | Rights::MANAGE_ROUTING
        | Rights::MANAGE_TICKETS
        | Rights::MANAGE_USERS
        | Rights::ADD_MEMBERS
        | Rights::BYPASS_RATE_LIMITS
        | Rights::CREATE_APPLICATIONS
        | Rights::CREATE_CHANNELS
        | Rights::CREATE_DMS
        | Rights::CREATE_DM_GROUPS
        | Rights::CREATE_GUILDS
        | Rights::CREATE_INVITES
        | Rights::CREATE_ROLES
        | Rights::CREATE_TEMPLATES
        | Rights::CREATE_WEBHOOKS
        | Rights::JOIN_GUILDS
        | Rights::PIN_MESSAGES
        | Rights::SELF_ADD_REACTIONS
        | Rights::SELF_DELETE_MESSAGES
        | Rights::SELF_EDIT_MESSAGES
        | Rights::SELF_EDIT_NAME
        | Rights::SEND_MESSAGES
        | Rights::USE_ACTIVITIES
        | Rights::USE_VIDEO
        | Rights::USE_VOICE
        | Rights::INVITE_USERS
        | Rights::SELF_DELETE_DISABLE
        | Rights::DEBTABLE
        | Rights::CREDITABLE
        | Rights::KICK_BAN_MEMBERS
        | Rights::SELF_LEAVE_GROUPS
        | Rights::PRESENCE
        | Rights::SELF_ADD_DISCOVERABLE
        | Rights::MANAGE_GUILD_DIRECTORY
        | Rights::POGGERS
        | Rights::USE_ACHIEVEMENTS
        | Rights::INITIATE_INTERACTIONS
        | Rights::RESPOND_TO_INTERACTIONS
        | Rights::SEND_BACKDATED_EVENTS
        | Rights::USE_MASS_INVITES
        | Rights::ACCEPT_INVITES
        | Rights::SELF_EDIT_FLAGS
        | Rights::EDIT_FLAGS
        | Rights::MANAGE_GROUPS
        | Rights::VIEW_SERVER_STATS
        | Rights::RESEND_VERIFICATION_EMAIL
}

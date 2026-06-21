use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::{LegacyRankColor, SupportRank};

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    application: Application,
    profiles: HashMap<Uuid, IdentityProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    client_id: String,
    scopes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProfile {
    username: String,
    primary: bool,
    rank: String,
    support_rank: SupportRank,
    #[serde(alias = "legacyRankColour")]
    legacy_rank_color: LegacyRankColor,
    rank_badge: String,
    access_rules: AccessRules,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessRules {
    identify: AccessRule,
    main_access: AccessRule,
    character_list_access: AccessRule,
    character_data_access: AccessRule,
    character_build_access: AccessRule,
    online_status: AccessRule,
    hunted_character_access: AccessRule,
    guild_history_access: AccessRule,
    guild_high_ranked_access: AccessRule,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessRule {
    Public,
    Private,
    Friend,
    Guild,
    GuildAndFriends,
    GuildOrFriends,
}

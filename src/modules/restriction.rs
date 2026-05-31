use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restrictions {
    #[serde(default)]
    pub main_access: bool,
    #[serde(default)]
    pub character_list_access: bool,
    #[serde(default)]
    pub character_data_access: bool,
    #[serde(default)]
    pub character_build_access: bool,
    #[serde(default)]
    pub hunted_character_access: bool,
    #[serde(default)]
    pub online_status: bool,
    #[serde(default)]
    pub guild_history_access: bool,
    #[serde(default)]
    #[serde(rename = "guild_high_ranked_access")]
    pub guild_high_ranked_access: bool,
}

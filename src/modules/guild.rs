use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub uuid: Uuid,
    pub name: String,
    pub prefix: String,
    pub level: u32,
    pub xp_percent: u32,
    pub territories: u32,
    pub wars: u32,
    pub raids: u32,
    pub created: DateTime<Utc>,
    pub members: Members,
    pub online: u32,
    pub banner: Banner,
    pub season_ranks: HashMap<String, SeasonRank>,
    pub ranking: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Members {
    pub total: u32,
    #[serde(default)]
    pub owner: HashMap<String, Member>,
    #[serde(default)]
    pub chief: HashMap<String, Member>,
    #[serde(default)]
    pub strategist: HashMap<String, Member>,
    #[serde(default)]
    pub captain: HashMap<String, Member>,
    #[serde(default)]
    pub recruiter: HashMap<String, Member>,
    #[serde(default)]
    pub recruit: HashMap<String, Member>,
}

impl Members {
    /// Iterate over every member alongside their rank, regardless of tier.
    pub fn all(&self) -> impl Iterator<Item = (GuildRank, &str, &Member)> {
        let tiers = [
            (GuildRank::Owner, &self.owner),
            (GuildRank::Chief, &self.chief),
            (GuildRank::Strategist, &self.strategist),
            (GuildRank::Captain, &self.captain),
            (GuildRank::Recruiter, &self.recruiter),
            (GuildRank::Recruit, &self.recruit),
        ];
        tiers.into_iter().flat_map(|(rank, map)| {
            map.iter()
                .map(move |(name, member)| (rank, name.as_str(), member))
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildRank {
    Owner,
    Chief,
    Strategist,
    Captain,
    Recruiter,
    Recruit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub uuid: Uuid,
    #[serde(alias = "legacyName")]
    pub username: String,
    pub online: bool,
    pub server: Option<String>,
    pub last_join: Option<DateTime<Utc>>,
    pub joined: DateTime<Utc>,
    pub contributed: u64,
    pub contribution_rank: u32,
    pub weekly: WeeklyChallenge,
    pub global_data: MemberGlobalData,
    pub restrictions: Restrictions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyChallenge {
    pub completed: Option<bool>,
    pub streak: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberGlobalData {
    pub content_completion: Option<u32>,
    pub wars: Option<u32>,
    pub total_level: Option<u32>,
    pub mobs_killed: Option<u32>,
    pub chests_found: Option<u32>,
    pub dungeons: Option<DungeonRaidList<DungeonName>>,
    pub raids: Option<DungeonRaidList<RaidName>>,
    pub world_events: Option<u32>,
    pub lootruns: Option<u32>,
    pub caves: Option<u32>,
    pub completed_quests: Option<u32>,
    pub pvp: Option<PvP>,
    pub current_guild_raids: Option<DungeonRaidList<RaidName>>,
    pub guild_raids: Option<DungeonRaidList<RaidName>>,
    pub playtime: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub base: BannerColor,
    pub tier: u32,
    pub structure: String,
    pub layers: Vec<BannerLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BannerColor {
    Black,
    Red,
    Green,
    Brown,
    Blue,
    Purple,
    Cyan,
    Silver,
    Gray,
    Pink,
    Lime,
    Yellow,
    LightBlue,
    Magenta,
    Orange,
    White,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerLayer {
    #[serde(alias = "colour")]
    pub color: BannerColor,
    pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonRank {
    pub rating: u32,
    pub final_territories: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const GUILD: &str = include_str!("../../tests/guild.json");

    #[test]
    fn guild_deserializes() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        assert_eq!(guild.name, "Spectral Cabbage");
        assert_eq!(guild.prefix, "SPC");
        assert_eq!(guild.level, 70);
        assert_eq!(guild.members.total, 5);
    }

    #[test]
    fn banner_color_screaming_snake_case() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        assert_eq!(guild.banner.base, BannerColor::Blue);
        assert_eq!(guild.banner.layers[0].color, BannerColor::White);
        assert_eq!(guild.banner.layers[1].color, BannerColor::Magenta);
    }

    #[test]
    fn season_ranks_parsed() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        assert!(guild.season_ranks.contains_key("1"));
        assert_eq!(guild.season_ranks["1"].rating, 10018);
    }

    #[test]
    fn members_all_iterator() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        let all: Vec<_> = guild.members.all().collect();
        assert_eq!(all.len(), 5);
        assert!(all.iter().any(|(rank, _, _)| *rank == GuildRank::Owner));
    }

    #[test]
    fn member_fields() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        let member = guild.members.owner.get("Nepmia").unwrap();
        assert_eq!(member.username, "Nepmia");
        assert_eq!(member.contributed, 64700439);
        assert!(!member.restrictions.online_status);
        assert!(member.restrictions.guild_high_ranked_access);
    }

    #[test]
    fn member_global_data() {
        let guild: Guild = serde_json::from_str(GUILD).unwrap();
        let data = &guild.members.owner.get("Nepmia").unwrap().global_data;
        assert_eq!(data.wars.unwrap(), 6);
        assert_eq!(data.playtime.unwrap(), 2485.42);
        assert_eq!(data.guild_raids.as_ref().unwrap().total, 4);
    }
}

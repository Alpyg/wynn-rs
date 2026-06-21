use std::collections::HashMap;

use derive_more::Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;

#[derive(Debug, Deref, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characters(pub HashMap<Uuid, Character>);

#[derive(Debug, Deref, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummaries(pub HashMap<Uuid, CharacterSummary>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummary {
    #[serde(rename = "type")]
    pub class: Class,
    pub reskin: Option<Reskin>,
    pub nickname: Option<String>,
    pub level: u32,
    pub xp: u64,
    pub xp_percent: u8,
    pub total_level: u32,
    pub gamemode: Vec<Gamemode>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    #[serde(rename = "type")]
    pub class: Class,
    pub reskin: Option<Reskin>,
    pub nickname: Option<String>,
    pub level: u32,
    pub xp: u64,
    pub xp_percent: u8,
    pub total_level: u32,
    pub gamemode: Vec<Gamemode>,
    pub content_completion: u32,
    pub wars: u32,
    pub playtime: f64,
    pub mobs_killed: u64,
    pub chests_found: u32,
    pub items_identified: u32,
    pub blocks_walked: u64,
    pub logins: u32,
    pub deaths: u32,
    pub discoveries: u32,
    pub pvp: PvP,
    pub skill_points: SkillPoints,
    pub professions: Professions,
    pub dungeons: DungeonRaidList<DungeonName>,
    pub raids: DungeonRaidList<RaidName>,
    pub world_events: u32,
    pub lootruns: u32,
    pub caves: u32,
    pub quests: Vec<String>,
    pub restrictions: Restrictions,
    #[serde(default)]
    pub removed_stat: Vec<serde_json::Value>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// True if this character has died on Hardcore mode and is now locked.
    pub died: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Class {
    Archer,
    Assassin,
    Mage,
    Shaman,
    Warrior,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reskin {
    Archer,
    Ninja,
    Darkwizard,
    Skyseer,
    Knight,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gamemode {
    Ironman,
    Craftsman,
    Hunted,
    Hardcore,
    Ultimate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillPoints {
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub defence: u32,
    pub agility: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Professions {
    pub fishing: ProfessionLevel,
    pub woodcutting: ProfessionLevel,
    pub mining: ProfessionLevel,
    pub farming: ProfessionLevel,
    pub scribing: ProfessionLevel,
    pub jeweling: ProfessionLevel,
    pub alchemism: ProfessionLevel,
    pub cooking: ProfessionLevel,
    pub weaponsmithing: ProfessionLevel,
    pub tailoring: ProfessionLevel,
    pub woodworking: ProfessionLevel,
    pub armouring: ProfessionLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfessionLevel {
    pub level: u32,
    pub xp_percent: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHARACTERS: &str = include_str!("../../tests/player_characters.json");
    const CHARACTER: &str = include_str!("../../tests/player_character.json");
    #[test]
    fn character_list_deserializes() {
        let summaries: CharacterSummaries = serde_json::from_str(CHARACTERS).unwrap();
        let c = summaries.values().next().unwrap();
        assert_eq!(c.class, Class::Assassin);
        assert_eq!(c.level, 106);
    }

    #[test]
    fn character_deserializes() {
        let c: Character = serde_json::from_str(CHARACTER).unwrap();
        assert_eq!(c.class, Class::Assassin);
        assert_eq!(c.level, 106);
    }
}

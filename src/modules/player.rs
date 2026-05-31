use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, de};
use uuid::Uuid;

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub username: String,
    pub online: bool,
    pub server: Option<String>,
    pub active_character: Option<Uuid>,
    pub nickname: Option<String>,
    pub uuid: Uuid,
    pub rank: String,
    pub rank_badge: String,
    #[serde(rename = "legacyRankColour")]
    pub legacy_rank_color: LegacyRankColor,
    pub shortened_rank: String,
    #[serde(default, deserialize_with = "deserialize_support_rank_opt")]
    pub support_rank: Option<SupportRank>,
    pub veteran: bool,
    pub last_join: DateTime<Utc>,
    pub guild: Option<PlayerGuild>,
    pub first_join: DateTime<Utc>,
    pub playtime: f64,
    pub global_data: GlobalData,
    pub wallpaper: String,
    pub avatar: String,
    pub characters: Option<Characters>,
    pub restrictions: Restrictions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerGuild {
    pub uuid: Uuid,
    pub name: String,
    pub prefix: String,
    pub rank: Option<String>,
    pub rank_stars: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalData {
    pub content_completion: u32,
    pub wars: u32,
    pub total_level: u32,
    pub mobs_killed: u32,
    pub chests_found: u32,
    pub dungeons: DungeonRaidList<DungeonName>,
    pub raids: DungeonRaidList<RaidName>,
    pub world_events: u32,
    pub lootruns: u32,
    pub caves: u32,
    pub completed_quests: u32,
    pub guild_raids: Option<DungeonRaidList<RaidName>>,
    pub raid_stats: Option<RaidStats>,
    pub pvp: PvP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonRaidList<K>
where
    K: DeserializeStringKey,
{
    pub total: u32,
    #[serde(deserialize_with = "deserialize_string_key_map")]
    pub list: HashMap<K, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum DungeonName {
    // Standard
    DecrepitSewers,
    InfestedPit,
    UnderworldCrypt,
    TimelostSanctum,
    SandSweptTomb,
    IceBarrows,
    UndergrowthRuins,
    GalleonsGraveyard,
    FallenFactory,
    EldritchOutlook,
    // Corrupted
    CorruptedDecrepitSewers,
    CorruptedInfestedPit,
    CorruptedLostSanctuary,
    CorruptedUnderworldCrypt,
    CorruptedSandSweptTomb,
    CorruptedIceBarrows,
    CorruptedUndergrowthRuins,
    CorruptedGalleonsGraveyard,
    // Legacy (removed content, kept for historical data)
    LegacySkeleton,
    LegacySpider,
    LegacyAnimal,
    LegacyZombie,
    LegacySilverfish,
    LegacyIce,
    LegacyOcean,
    LegacyJungle,
    // Forward-compat
    Unknown(String),
}

impl FromStr for DungeonName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Decrepit Sewers" => Self::DecrepitSewers,
            "Infested Pit" => Self::InfestedPit,
            "Underworld Crypt" => Self::UnderworldCrypt,
            "Timelost Sanctum" => Self::TimelostSanctum,
            "Sand-Swept Tomb" => Self::SandSweptTomb,
            "Ice Barrows" => Self::IceBarrows,
            "Undergrowth Ruins" => Self::UndergrowthRuins,
            "Galleon's Graveyard" => Self::GalleonsGraveyard,
            "Fallen Factory" => Self::FallenFactory,
            "Eldritch Outlook" => Self::EldritchOutlook,
            "Corrupted Decrepit Sewers" => Self::CorruptedDecrepitSewers,
            "Corrupted Infested Pit" => Self::CorruptedInfestedPit,
            "Corrupted Lost Sanctuary" => Self::CorruptedLostSanctuary,
            "Corrupted Underworld Crypt" => Self::CorruptedUnderworldCrypt,
            "Corrupted Sand-Swept Tomb" => Self::CorruptedSandSweptTomb,
            "Corrupted Ice Barrows" => Self::CorruptedIceBarrows,
            "Corrupted Undergrowth Ruins" => Self::CorruptedUndergrowthRuins,
            "Corrupted Galleon's Graveyard" => Self::CorruptedGalleonsGraveyard,
            "Skeleton" => Self::LegacySkeleton,
            "Spider" => Self::LegacySpider,
            "Animal" => Self::LegacyAnimal,
            "Zombie" => Self::LegacyZombie,
            "Silverfish" => Self::LegacySilverfish,
            "Ice" => Self::LegacyIce,
            "Ocean" => Self::LegacyOcean,
            "Jungle" => Self::LegacyJungle,
            other => Self::Unknown(other.to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum RaidName {
    NestOfTheGrootslangs,
    OrphionsNexusOfLight,
    TheCanyonColossus,
    TheNamelessAnomaly,
    TheWartornPalace,
    // Forward-compat
    Unknown(String),
}

impl FromStr for RaidName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Nest of the Grootslangs" => Self::NestOfTheGrootslangs,
            "Orphion's Nexus of Light" => Self::OrphionsNexusOfLight,
            "The Canyon Colossus" => Self::TheCanyonColossus,
            "The Nameless Anomaly" => Self::TheNamelessAnomaly,
            "The Wartorn Palace" => Self::TheWartornPalace,
            other => Self::Unknown(other.to_string()),
        })
    }
}

pub trait DeserializeStringKey:
    Eq + std::hash::Hash + FromStr<Err = std::convert::Infallible>
{
}

impl DeserializeStringKey for DungeonName {}
impl DeserializeStringKey for RaidName {}

fn deserialize_string_key_map<'de, D, K>(deserializer: D) -> Result<HashMap<K, u32>, D::Error>
where
    D: Deserializer<'de>,
    K: DeserializeStringKey,
{
    struct StringKeyMapVisitor<K>(std::marker::PhantomData<K>);

    impl<'de, K> de::Visitor<'de> for StringKeyMapVisitor<K>
    where
        K: DeserializeStringKey,
    {
        type Value = HashMap<K, u32>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a map with string keys")
        }

        fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut out = HashMap::new();
            while let Some((raw, count)) = map.next_entry::<String, u32>()? {
                // Infallible unwrap — FromStr<Err = Infallible> can never fail
                let key = K::from_str(&raw).unwrap();
                out.insert(key, count);
            }
            Ok(out)
        }
    }

    deserializer.deserialize_map(StringKeyMapVisitor(std::marker::PhantomData))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaidStats {
    pub damage_taken: u64,
    pub damage_dealt: u64,
    pub health_healed: u64,
    pub deaths: u32,
    pub buffs_taken: u32,
    pub gambits_used: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PvP {
    pub kills: u32,
    pub deaths: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyRankColor {
    pub main: String,
    pub sub: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SupportRank {
    Vip,
    VipPlus,
    Hero,
    HeroPlus,
    Champion,
}

impl<'de> Deserialize<'de> for SupportRank {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.to_ascii_lowercase().as_str() {
            "vip" => Ok(Self::Vip),
            "vip+" => Ok(Self::VipPlus),
            "hero" => Ok(Self::Hero),
            "hero+" => Ok(Self::HeroPlus),
            "champion" => Ok(Self::Champion),
            other => Err(de::Error::unknown_variant(
                other,
                &["vip", "vip+", "hero", "hero+", "champion"],
            )),
        }
    }
}

fn deserialize_support_rank_opt<'de, D>(d: D) -> Result<Option<SupportRank>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<SupportRank>::deserialize(d)
}

#[cfg(test)]
mod tests {
    use crate::modules::character::Class;

    use super::*;

    const SAMPLE: &str = include_str!("../../tests/player_profile.json");

    #[test]
    fn parses_full_profile() {
        let profile: PlayerProfile =
            serde_json::from_str(SAMPLE).expect("deserialization should succeed");

        assert_eq!(profile.username, "Nepmia");
        assert!(!profile.online);
        assert_eq!(profile.support_rank, Some(SupportRank::Champion));

        let guild = profile.guild.as_ref().unwrap();
        assert_eq!(guild.prefix, "SPC");

        let global = &profile.global_data;
        assert_eq!(global.dungeons.total, 2);
        assert_eq!(
            global.dungeons.list.get(&DungeonName::DecrepitSewers),
            Some(&1)
        );
        assert_eq!(
            global.raids.list.get(&RaidName::NestOfTheGrootslangs),
            Some(&1)
        );

        let characters = profile.characters.unwrap();
        let (_, character) = characters.iter().next().unwrap();
        assert_eq!(character.class, Class::Assassin);
        assert_eq!(character.level, 106);
        assert_eq!(character.professions.farming.level, 91);
    }

    #[test]
    fn unknown_dungeon_preserved() {
        let json = r#"{"total":1,"list":{"Brand New Dungeon":1}}"#;
        let parsed: DungeonRaidList<DungeonName> = serde_json::from_str(json).unwrap();
        assert!(matches!(
            parsed.list.keys().next().unwrap(),
            DungeonName::Unknown(_)
        ));
    }

    #[test]
    fn support_rank_case_insensitive() {
        for input in ["champion", "Champion", "CHAMPION"] {
            let json = format!("\"{input}\"");
            let rank: SupportRank = serde_json::from_str(&json).unwrap();
            assert_eq!(rank, SupportRank::Champion);
        }
    }
}

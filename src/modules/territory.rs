use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_more::{Deref, IntoIterator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Deref, IntoIterator)]
#[serde(rename_all = "camelCase")]
pub struct Territories(pub HashMap<String, Territory>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Territory {
    pub guild: TerritoryGuild,
    pub acquired: DateTime<Utc>,
    pub hq: bool,
    pub resources: Vec<Resource>,
    pub links: Vec<String>,
    pub treasury: TerritoryLevel,
    pub defences: TerritoryLevel,
    pub location: TerritoryLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryGuild {
    pub uuid: Uuid,
    pub name: String,
    pub prefix: String,
    pub hq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub r#type: ResourceType,
    pub generation: i32,
    pub stored: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    Emerald,
    Ore,
    Wood,
    Fish,
    Crop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerritoryLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerritoryLocation {
    pub start: Vec<i32>,
    pub end: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TERRITORIES: &str = include_str!("../../tests/territories.json");

    #[test]
    fn deserialize_territories() {
        _ = serde_json::from_str::<Territories>(TERRITORIES).unwrap();
    }
}

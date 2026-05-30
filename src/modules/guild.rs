use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Guild {
    pub uuid: Uuid,
    pub name: String,
    pub prefix: String,
    pub rank: Option<String>,
    pub rank_stars: Option<String>,
}

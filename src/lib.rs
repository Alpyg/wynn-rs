pub mod client;
pub mod modules;

pub use crate::client::*;

pub use crate::modules::auth::*;
pub use crate::modules::character::*;
pub use crate::modules::guild::*;
pub use crate::modules::player::*;
pub use crate::modules::restriction::*;

pub type PlayerResponse = PlayerProfile;
pub type CharacterListResponse = CharacterSummaries;
pub type CharacterResponse = Character;

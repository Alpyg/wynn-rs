pub mod client;
pub mod error;
pub mod modules;

pub use crate::client::*;
pub use crate::error::*;
pub use crate::modules::auth::*;
pub use crate::modules::character::*;
pub use crate::modules::guild::*;
pub use crate::modules::player::*;
pub use crate::modules::restriction::*;
pub use crate::modules::territory::*;

pub type PlayerResponse = PlayerProfile;
pub type CharacterListResponse = CharacterSummaries;
pub type CharacterResponse = Character;

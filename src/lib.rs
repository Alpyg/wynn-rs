pub mod modules;

pub mod prelude {
    pub use crate::modules::character::*;
    pub use crate::modules::player::*;
    pub use crate::modules::restriction::*;

    pub type PlayerResponse = PlayerProfile;
    pub type CharacterListResponse = CharacterSummaries;
    pub type CharacterResponse = Character;
}

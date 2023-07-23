use serde::{Deserialize, Serialize};

/// Module containing structures related to character achievements.
pub mod achievements;
/// Module containing structures related to character classes.
pub mod class;
/// Module containing structures related to character gear sets.
pub mod gear;

use crate::{freecompany::FreeCompany, pagination::Pagination};
use achievements::CharacterAchievements;

use self::{
    class::{Class, ClassBozjan, ClassElemental},
    gear::GearSet,
};

/// Collection of characters that match a name search.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterSearchResults {
    /// Pagination information for the search results.
    pagination: Pagination,
    /// List of characters that match the name search.
    pub results: Vec<CharacterSearch>,
}

/// All information obtained about a character from a name search.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterSearch {
    /// The URL of the character's avatar.
    pub avatar: String,
    /// The number of Feast matches the character has participated in.
    pub feast_matches: u32,
    /// The unique ID of the character.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The language code associated with the character, if available.
    pub lang: Option<String>,
    /// The name of the character.
    pub name: String,
    /// The rank of the character, if available.
    pub rank: Option<String>,
    /// The URL of the rank icon for the character, if available.
    pub rank_icon: Option<String>,
    /// The name of the server the character belongs to.
    pub server: String,
}

/// All information obtained from a character ID request.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterResult {
    /// Detailed information about the character's achievements, if available.
    pub achievements: Option<CharacterAchievements>,
    /// Indicates if the character's achievements are public or not.
    pub achievements_public: Option<bool>,
    /// Detailed information about the character.
    pub character: Character,
    /// Detailed information about the Free Company the character belongs to, if available.
    pub free_company: Option<FreeCompany>,
    /// List of members of the Free Company the character belongs to, if available.
    pub free_company_members: Option<Vec<CharacterSearch>>,
    /// List of the character's friends, if available.
    pub friends: Option<Vec<CharacterSearch>>,
    /// Indicates if the character's friend list is public or not.
    pub friends_public: Option<bool>,
    /// List of the character's minions, if available.
    pub minions: Option<Vec<Mimo>>,
    /// List of the character's mounts, if available.
    pub mounts: Option<Vec<Mimo>>,
    /// The ID of the character's PvP Team, if available.
    #[serde(rename = "PvPTeam")]
    pub pvpteam: Option<String>,
}

/// A detailed character profile.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    /// Detailed information about the character's active class job.
    pub active_class_job: Class,
    /// The URL of the character's avatar.
    pub avatar: String,
    /// The character's biography.
    pub bio: String,
    /// List of class jobs the character has.
    pub class_jobs: Vec<Class>,
    /// Class job information related to Bozjan content.
    pub class_jobs_bozjan: ClassBozjan,
    /// Class job information related to Elemental content.
    pub class_jobs_elemental: ClassElemental,
    /// The data center the character belongs to.
    #[serde(rename = "DC")]
    pub dc: String,
    /// The ID of the Free Company the character belongs to.
    pub free_company_id: String,
    /// The name of the Free Company the character belongs to.
    pub free_company_name: String,
    /// Detailed information about the character's gear set.
    pub gear_set: GearSet,
    /// The character's gender.
    pub gender: u8,
    /// Detailed information about the character's Grand Company affiliation.
    pub grand_company: GrandCompany,
    /// The ID of the character.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The language code associated with the character, if available.
    pub lang: Option<String>,
    /// The name of the character.
    pub name: String,
    /// The character's nameday.
    pub nameday: String,
    /// The parse date for the character's data.
    pub parse_date: u32,
    /// The URL of the character's portrait.
    pub portrait: String,
    /// The ID of the character's PvP Team, if available.
    #[serde(rename = "PvPTeamId")]
    pub pvp_team_id: Option<String>,
    /// The character's race.
    pub race: u8,
    /// The name of the server the character belongs to.
    pub server: String,
    /// The ID of the character's title, if available.
    pub title: u32,
    /// Indicates if the character's title is at the top or bottom, if available.
    pub title_top: bool,
    /// The ID of the town the character is associated with.
    pub town: u32,
    /// The ID of the tribe the character is associated with.
    pub tribe: u32,
}

/// A character's Grand Company information.
#[derive(Deserialize, Serialize, Debug)]
pub struct GrandCompany {
    /// The ID of the Grand Company's name.
    #[serde(rename = "NameID")]
    pub name_id: u8,
    /// The ID of the Grand Company's rank.
    #[serde(rename = "RankID")]
    pub rank_id: u8,
}

/// Mounts and minions of a character.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Mimo {
    /// The URL of the icon representing the mount or minion.
    pub icon: String,
    /// The name of the mount or minion.
    pub name: String,
}
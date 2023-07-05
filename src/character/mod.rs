use serde::{Deserialize, Serialize};

pub mod achievements;
pub mod class;
pub mod gear;

use crate::{freecompany::FreeCompany, pagination::Pagination};
use achievements::CharacterAchievements;

use self::{
    class::{Class, ClassBozjan, ClassElemental},
    gear::GearSet,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Collection of characters that match a name search.
pub struct CharacterSearchResults {
    pagination: Pagination,
    pub results: Vec<CharacterSearch>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// All information obtained about a character from a name search.
pub struct CharacterSearch {
    pub avatar: String,
    pub feast_matches: u32,
    #[serde(rename = "ID")]
    pub id: u32,
    pub lang: Option<String>,
    pub name: String,
    pub rank: Option<String>,
    pub rank_icon: Option<String>,
    pub server: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// All information gained from a character ID request.
pub struct CharacterResult {
    pub achievements: Option<CharacterAchievements>,
    pub achievements_public: Option<bool>,
    pub character: Character,
    pub free_company: Option<FreeCompany>,
    pub free_company_members: Option<Vec<CharacterSearch>>,
    pub friends: Option<Vec<CharacterSearch>>,
    pub friends_public: Option<bool>,
    pub minions: Option<Vec<Mimo>>,
    pub mounts: Option<Vec<Mimo>>,
    #[serde(rename = "PvPTeam")]
    pub pvpteam: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// A detailed character profile.
pub struct Character {
    pub active_class_job: Class,
    pub avatar: String,
    pub bio: String,
    pub class_jobs: Vec<Class>,
    pub class_jobs_bozjan: ClassBozjan,
    pub class_jobs_elemental: ClassElemental,
    #[serde(rename = "DC")]
    pub dc: String,
    pub free_company_id: String,
    pub free_company_name: String,
    pub gear_set: GearSet,
    pub gender: u8,
    pub grand_company: GrandCompany,
    pub guardian_deity: u8,
    #[serde(rename = "ID")]
    pub id: u32,
    pub lang: Option<String>,
    pub name: String,
    pub nameday: String,
    pub parse_date: u32,
    pub portrait: String,
    #[serde(rename = "PvPTeamId")]
    pub pvp_team_id: Option<String>,
    pub race: u8,
    pub server: String,
    pub title: u32,
    pub title_top: bool,
    pub town: u32,
    pub tribe: u32,
}

#[derive(Deserialize, Serialize, Debug)]
/// A character's grand company.
pub struct GrandCompany {
    #[serde(rename = "NameID")]
    pub name_id: u8,
    #[serde(rename = "RankID")]
    pub rank_id: u8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Mounts and minions of a character
pub struct Mimo {
    pub icon: String,
    pub name: String,
}

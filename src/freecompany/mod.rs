use serde::{Deserialize, Serialize};

use crate::{character::CharacterSearch, pagination::Pagination};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Collection of Free Companies that match a name search.
pub struct FreeCompanySearchResults {
    pagination: Pagination,
    pub results: Vec<FreeCompanySearch>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// All information obtained from a Free Company from a name search.
pub struct FreeCompanySearch {
    pub crest: Vec<String>,
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub server: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompanyResult {
    pub free_company: FreeCompany,
    pub free_company_members: Option<Vec<CharacterSearch>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Details about a Free Company (FC).
pub struct FreeCompany {
    pub active: String,
    pub active_member_count: u16,
    pub crest: Vec<String>,
    #[serde(rename = "DC")]
    pub dc: String,
    pub estate: FcEstate,
    pub focus: Vec<FcFocus>,
    pub formed: u32,
    pub grand_company: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub parse_date: u32,
    pub rank: u8,
    pub ranking: FcRanking,
    pub recruitment: String,
    pub reputation: Vec<FcReputation>,
    pub seeking: Vec<Seeking>,
    pub server: String,
    pub slogan: String,
    pub tag: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Info about an FC's estate.
pub struct FcEstate {
    pub greeting: String,
    pub name: String,
    pub plot: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// An FC's chosen focus.
pub struct FcFocus {
    pub icon: String,
    pub name: String,
    pub status: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// An FC's montly and yearly ranking.
pub struct FcRanking {
    pub monthly: u32,
    pub weekly: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// An FC's standing with Eorzea's nations.
pub struct FcReputation {
    pub name: String,
    pub progress: u8,
    pub rank: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Seeking {
    pub icon: String,
    pub name: String,
    pub status: bool,
}

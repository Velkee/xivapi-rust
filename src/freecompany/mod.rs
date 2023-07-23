use serde::{Deserialize, Serialize};

use crate::{character::CharacterSearch, pagination::Pagination};

/// Collection of Free Companies that match a name search.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompanySearchResults {
    /// Pagination information fro the search resuts.
    pub pagination: Pagination,
    /// List of Free Companies that match the search criteria.
    pub results: Vec<FreeCompanySearch>,
}

/// Detailed information about a Free Company obtained from a name search.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompanySearch {
    /// List of the images used to make up a Free Company's crest icon.
    pub crest: Vec<String>,
    /// The ID of the Free Company.
    #[serde(rename = "ID")]
    pub id: String,
    /// The name of the Free Company.
    pub name: String,
    /// The server on which the Free Company exists.
    pub server: String,
}

/// Detailed information about a spesific Free Company.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompanyResult {
    /// Information about the Free Company itself.
    pub free_company: FreeCompany,
    /// Optional list of members belonging to the Free Company. Only returned if `extended` is set to `true` while calling `free_company_lookup`
    pub free_company_members: Option<Vec<CharacterSearch>>,
}

/// Details about a Free Company.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompany {
    /// The active status of the Free Company.
    pub active: String,
    /// The count of active members in the Free Company.
    pub active_member_count: u16,
    /// List of the images used to make up a Free Company's crest icon.
    pub crest: Vec<String>,
    /// The data center where the Free Company's server is located.
    #[serde(rename = "DC")]
    pub dc: String,
    /// Information about the Free Company's estate (if applicable).
    pub estate: FcEstate,
    /// List of the Free Company's chosen foci.
    pub focus: Vec<FcFocus>,
    /// Year in witch the Free Company was formed.
    pub formed: u32,
    /// The Grand Company affiliation of the Free Company.
    pub grand_company: String,
    /// the ID of the Free Company.
    #[serde(rename = "ID")]
    pub id: String,
    /// The name of the Free Company
    pub name: String,
    /// The date when the data was last parsed or updated.
    pub parse_date: u32,
    /// The rank of the Free Company.
    pub rank: u8,
    /// Ranking information for the Free Company
    pub ranking: FcRanking,
    /// Information about the Free Company's recruitment status.
    pub recruitment: String,
    /// Lis of reputations with Eorzea's nations.
    pub reputation: Vec<FcReputation>,
    /// List of the roles the Free Company is looking for.
    pub seeking: Vec<Seeking>,
    /// The server where the Free Company is located.
    pub server: String,
    /// The slogan of the Free Company.
    pub slogan: String,
    /// The tag associated with the Free Company.
    pub tag: String,
}

/// Information about an FC's estate (if applicable).
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FcEstate {
    /// The greeting message for the estate.
    pub greeting: String,
    /// The name of the estate.
    pub name: String,
    /// The plot, ward, and housing area where the estate is located.
    pub plot: String,
}

/// An FC's chosen focus.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FcFocus {
    /// The icon representing the focus.
    pub icon: String,
    /// The name of the focus.
    pub name: String,
    /// The status of the focus.
    pub status: bool,
}

/// Information about an FC's monthly and yearly ranking.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FcRanking {
    /// The monthly ranking of the FC.
    pub monthly: u32,
    /// The weekly ranking of the FC.
    pub weekly: u32,
}

/// Information about an FC's standing with Eorzea's nations.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FcReputation {
    /// The name of the nation.
    pub name: String,
    /// THe progress level of the FC with the nation.
    pub progress: u8,
    /// The rank of the FC with the nation.
    pub rank: String,
}

/// Information about a spesific FC's seeked roles.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Seeking {
    /// The icon representing the role.
    pub icon: String,
    /// The name of the role.
    pub name: String,
    /// Wether or not the role is being seeked.
    pub status: bool,
}

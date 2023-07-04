use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    #[serde(rename = "ClassID")]
    pub class_id: u32,
    pub exp_level: u32,
    pub exp_level_max: u32,
    pub exp_level_togo: u32,
    pub is_specialised: bool,
    #[serde(rename = "JobID")]
    pub job_id: u32,
    pub level: u32,
    pub name: String,
    pub unlocked_state: ClassUnlockedState,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassUnlockedState {
    #[serde(rename = "ID")]
    pub id: Option<u32>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBozjan {
    pub level: Option<u32>,
    pub mettle: Option<u32>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassElemental {
    pub exp_level: u32,
    pub exp_level_max: u32,
    pub exp_level_togo: u32,
    pub level: u32,
    pub name: String,
}

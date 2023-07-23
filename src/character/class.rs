use serde::{Deserialize, Serialize};


/// Class information.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    /// The unique ID of the class.
    #[serde(rename = "ClassID")]
    pub class_id: u32,
    /// The current experience level of the class.
    pub exp_level: u32,
    /// The maximum experience level of the class.
    pub exp_level_max: u32,
    /// The remaining experience points required to reach the next level.
    pub exp_level_togo: u32,
    /// Indicates if the class is a specialized job or not.
    pub is_specialised: bool,
    /// The unique ID of the corresponding job, if the class has been upgraded to a job.
    #[serde(rename = "JobID")]
    pub job_id: u32,
    /// The current level of the class or job.
    pub level: u32,
    /// The name of the class.
    pub name: String,
    /// Information about the unlocked state of the class.
    pub unlocked_state: ClassUnlockedState,
}

/// Information about whether a class has been unlocked and, if so, whether it has been upgraded to a job.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassUnlockedState {
    /// The unique ID of the unlocked state, if available.
    #[serde(rename = "ID")]
    pub id: Option<u32>,
    /// The name of the unlocked state.
    pub name: String,
}

/// Class related to Bozjan content.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBozjan {
    /// The level of the class in Bozjan content, if applicable.
    pub level: Option<u32>,
    /// The mettle level of the class in Bozjan content, if applicable.
    pub mettle: Option<u32>,
    /// The name of the class related to Bozjan content.
    pub name: String,
}

/// Elemental class information.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClassElemental {
    /// The current experience level of the elemental class.
    pub exp_level: u32,
    /// The maximum experience level of the elemental class.
    pub exp_level_max: u32,
    /// The remaining experience points required to reach the next level for the elemental class.
    pub exp_level_togo: u32,
    /// The current level of the elemental class.
    pub level: u32,
    /// The name of the elemental class.
    pub name: String,
}
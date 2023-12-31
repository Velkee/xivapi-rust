use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Unlocked achievements.
///
/// Only returned if achievements are public.
pub struct CharacterAchievements {
    list: Vec<Achievements>,
    points: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Achievements {
    list: Vec<String>,
    points: u32,
}

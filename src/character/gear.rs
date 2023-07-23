use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A character's current gear set.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GearSet {
    /// A map of attribute IDs to their corresponding values for the gear set.
    pub attributes: HashMap<u32, u32>,
    /// The unique ID of the class associated with the gear set.
    #[serde(rename = "ClassID")]
    pub class_id: u32,
    /// The individual gear pieces equipped by the character.
    pub gear: Gear,
    /// The gear set's unique key identifier.
    pub gear_key: String,
    /// The unique ID of the job associated with the gear set (if applicable).
    #[serde(rename = "JobID")]
    pub job_id: u32,
    /// The character's current level for the gear set.
    pub level: u32,
}

/// Individual gear slots containing equipped gear pieces.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    /// The gear piece equipped in the body slot.
    pub body: Option<GearPiece>,
    /// The gear piece equipped in the bracelets slot.
    pub bracelets: Option<GearPiece>,
    /// The gear piece equipped in the earrings slot.
    pub earrings: Option<GearPiece>,
    /// The gear piece equipped in the feet slot.
    pub feet: Option<GearPiece>,
    /// The gear piece equipped in the hands slot.
    pub hands: Option<GearPiece>,
    /// The gear piece equipped in the head slot.
    pub head: Option<GearPiece>,
    /// The gear piece equipped in the legs slot.
    pub legs: Option<GearPiece>,
    /// The gear piece equipped in the main hand slot.
    pub main_hand: Option<GearPiece>,
    /// The gear piece equipped in the necklace slot.
    pub necklace: Option<GearPiece>,
    /// The gear piece equipped in the off-hand slot.
    pub off_hand: Option<GearPiece>,
    /// The gear piece equipped in the first ring slot.
    pub ring1: Option<GearPiece>,
    /// The gear piece equipped in the second ring slot.
    pub ring2: Option<GearPiece>,
}

/// Information about an individual gear piece.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GearPiece {
    /// The name of the creator of the gear piece (if applicable).
    pub creator: Option<String>,
    /// The unique ID of the dye applied to the gear piece (if applicable).
    pub dye: Option<u32>,
    /// The unique ID of the gear piece.
    #[serde(rename = "ID")]
    pub id: u32,
    /// A list of unique IDs corresponding to materia attached to the gear piece.
    pub materia: Vec<u32>,
    /// The unique ID of the mirage applied to the gear piece (if applicable).
    pub mirage: Option<u32>,
}
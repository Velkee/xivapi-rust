use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GearSet {
    pub attributes: HashMap<u32, u32>,
    #[serde(rename = "ClassID")]
    pub class_id: u32,
    pub gear: Gear,
    pub gear_key: String,
    #[serde(rename = "JobID")]
    pub job_id: u32,
    pub level: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    pub body: Option<GearPiece>,
    pub bracelets: Option<GearPiece>,
    pub earrings: Option<GearPiece>,
    pub feet: Option<GearPiece>,
    pub hands: Option<GearPiece>,
    pub head: Option<GearPiece>,
    pub legs: Option<GearPiece>,
    pub main_hand: Option<GearPiece>,
    pub necklace: Option<GearPiece>,
    pub off_hand: Option<GearPiece>,
    pub ring1: Option<GearPiece>,
    pub ring2: Option<GearPiece>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GearPiece {
    pub creator: Option<String>,
    pub dye: Option<u32>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub materia: Vec<u32>,
    pub mirage: Option<u32>,
}

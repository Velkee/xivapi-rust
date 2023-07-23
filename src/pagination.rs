use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    pub page: u8,
    pub page_next: Option<u8>,
    pub page_prev: Option<u8>,
    pub page_total: u8,
    pub results: u8,
    pub results_per_page: u8,
    pub results_total: u16,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    page: u8,
    page_next: Option<u8>,
    page_prev: Option<u8>,
    page_total: u8,
    results: u8,
    results_per_page: u8,
    results_total: u16,
}

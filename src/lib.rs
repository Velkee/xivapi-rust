//! This crate provides a library for interacting with [XIVAPI], a REST API for the popular MMORPG Final Fantasy XIV.
//!
//! XIVAPI's API docs can be found [here].
//!
//! [XIVAPI]: https://xivapi.com
//! [here]: https://xivapi.com/docs

use freecompany::{FreeCompanyResult, FreeCompanySearchResults};
use reqwest::Client;

/// Structs and modules used in character searches.
pub mod character;
/// Structs used to parse FC information.
pub mod freecompany;

mod pagination;

use character::{CharacterResult, CharacterSearchResults};

#[cfg(test)]
mod tests {
    use crate::XIVAPIClient;

    #[tokio::test]
    async fn test_character_search() {
        let client = XIVAPIClient::new();

        client
            .character_search("Tami Pesagniyah", Some("Omega"), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_character_lookup() {
        let client = XIVAPIClient::new();

        let result = client
            .character_search("Tami Pesagniyah", Some("Omega"), None)
            .await
            .unwrap();

        client
            .character_lookup(result.results[0].id, false, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_free_company_search() {
        let client = XIVAPIClient::new();

        client
            .free_company_search("SEES", Some("Omega"), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_free_company_lookup() {
        let client = XIVAPIClient::new();

        let result = client
            .free_company_search("SEES", Some("Omega"), None)
            .await
            .unwrap();

        client
            .free_company_lookup(&result.results[0].id, false, None)
            .await
            .unwrap();
    }
}

/// The main client. Responsible for running all API queries.
///
/// You must create a new client using `XIVAPIClient::new()` before you can make API calls.
///
/// # Examples
/// ```
/// use xivapi_rust::XIVAPIClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = XIVAPIClient::new();
///
///     // Queries XIVAPI for a character named "Scott" on the Omega server.
///     let result = client.character_search("Scott", Some("Omega"), None)
///         .await
///         .unwrap();
/// }
/// ```
pub struct XIVAPIClient {
    client: Client,
}

impl XIVAPIClient {
    /// Creates a new instance of XIVAPIClient.
    pub fn new() -> Self {
        let client = Client::new();
        XIVAPIClient { client }
    }

    /// Does a name-based search for a character.
    ///
    /// Returns basic info of all characters that match the search criteria.
    /// If no characters match the criteria, the returned array will be empty.
    pub async fn character_search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<u8>,
    ) -> Result<CharacterSearchResults, reqwest::Error> {
        let mut search_params = Vec::new();

        let name = name.split_whitespace().collect::<Vec<&str>>().join("+");
        search_params.push(format!("name={}", name));

        if let Some(server) = server {
            search_params.push(format!("server={}", server))
        }

        if let Some(page) = page {
            search_params.push(format!("page={}", page))
        }

        let search_string = format!(
            "https://xivapi.com/character/search?{}",
            search_params.join("&")
        );

        let result: CharacterSearchResults =
            self.client.get(search_string).send().await?.json().await?;

        Ok(result)
    }

    /// Gives detailed information about a character from a character ID.
    ///
    /// The `extended` option will extend out the data IDs of useful objects.
    pub async fn character_lookup(
        &self,
        character_id: u32,
        extended: bool,
        data: Option<Vec<&str>>,
    ) -> Result<CharacterResult, reqwest::Error> {
        let mut seach_params = Vec::new();

        if extended {
            seach_params.push("extended=1".to_string())
        }

        if let Some(data) = data {
            let data = data.join(",");

            seach_params.push(format!("data={}", data))
        }

        let search_string = format!(
            "https://xivapi.com/character/{}?{}",
            character_id,
            seach_params.join("&")
        );

        let result: CharacterResult = self.client.get(search_string).send().await?.json().await?;

        Ok(result)
    }

    pub async fn free_company_search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<u8>,
    ) -> Result<FreeCompanySearchResults, reqwest::Error> {
        let mut search_params = Vec::new();

        let name = name.split_whitespace().collect::<Vec<&str>>().join("+");
        search_params.push(format!("name={}", name));

        if let Some(server) = server {
            search_params.push(format!("server={}", server))
        }

        if let Some(page) = page {
            search_params.push(format!("page={}", page))
        }

        let search_string = format!(
            "https://xivapi.com/freecompany/search?{}",
            search_params.join("&")
        );

        let result: FreeCompanySearchResults =
            self.client.get(search_string).send().await?.json().await?;

        Ok(result)
    }

    pub async fn free_company_lookup(
        &self,
        character_id: &str,
        extended: bool,
        data: Option<Vec<&str>>,
    ) -> Result<FreeCompanyResult, reqwest::Error> {
        let mut seach_params = Vec::new();

        if extended {
            seach_params.push("extended=1".to_string())
        }

        if let Some(data) = data {
            let data = data.join(",");

            seach_params.push(format!("data={}", data))
        }

        let search_string = format!(
            "https://xivapi.com/freecompany/{}?{}",
            character_id,
            seach_params.join("&")
        );

        let result: FreeCompanyResult = self.client.get(search_string).send().await?.json().await?;

        Ok(result)
    }
}

impl Default for XIVAPIClient {
    fn default() -> Self {
        Self::new()
    }
}

//! This crate provides a library for interacting with [XIVAPI], a REST API for the popular MMORPG Final Fantasy XIV.
//!
//! XIVAPI's API docs can be found [here].
//!
//! [XIVAPI]: https://xivapi.com
//! [here]: https://xivapi.com/docs


#![warn(missing_docs)]

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
    async fn test_character_search() -> Result<(), reqwest::Error> {
        let client = XIVAPIClient::new();

        client
            .character_search("Tami Pesagniyah", Some("Omega"), None)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_character_lookup() -> Result<(), reqwest::Error> {
        let client = XIVAPIClient::new();

        let result = client
            .character_search("Tami Pesagniyah", Some("Omega"), None)
            .await?;

        client
            .character_lookup(result.results[0].id, false, None)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_free_company_search() -> Result<(), reqwest::Error> {
        let client = XIVAPIClient::new();

        client
            .free_company_search("SEES", Some("Omega"), None)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_free_company_lookup() -> Result<(), reqwest::Error> {
        let client = XIVAPIClient::new();

        let result = client
            .free_company_search("SEES", Some("Omega"), None)
            .await
            .unwrap();

        client
            .free_company_lookup(&result.results[0].id, false, None)
            .await?;

        Ok(())
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
/// async fn main() -> Result<(), reqwest::Error> {
///     let client = XIVAPIClient::new();
///
///     // Queries XIVAPI for a character named "Scott" on the Omega server.
///     let result = client.character_search("Scott", Some("Omega"), None)
///         .await?;
///
///     Ok(())
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
    /// Returns basic information of all characters that match the search criteria.
    /// If no characters match the criteria, the returned array will be empty.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the character to search for.
    /// * `server` - An optional server name filter for the character search.
    /// * `page` - An optional page number for paginated results.
    ///
    /// # Returns
    ///
    /// A `Result` containing the search results as `CharacterSearchResults` or a `reqwest::Error`
    /// if the request to XIVAPI fails.
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
    /// # Arguments
    /// 
    /// * `character_id` - The ID of the character to look up.
    /// * `extended` - If true, extends out the data IDs of useful objects.
    /// * `data` - Additional information to be requested from the API.
    /// 
    /// # Returns
    ///
    /// A `Result` containing character information as `CharacterResult` or a `reqwest::Error`
    /// if the request to XIVAPI fails.
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

    /// Does a name-based search for a Free Company (FC).
    ///
    /// Returns basic information of all Free Companies that match the search criteria.
    /// If no Free Companies match the criteria, the returned array will be empty.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Free Company to search for.
    /// * `server` - An optional server name filter for the Free Company search.
    /// * `page` - An optional page number for paginated results.
    ///
    /// # Returns
    ///
    /// A `Result` containing the search results as `FreeCompanySearchResults` or a `reqwest::Error`
    /// if the request to XIVAPI fails.
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

    /// Gives detailed information about a Free Company (FC) from a Free Company ID.
    ///
    /// The `extended` option will extend out additional data IDs of useful objects.
    ///
    /// # Arguments
    ///
    /// * `free_company_id` - The ID of the Free Company to look up.
    /// * `extended` - If true, extends out additional data IDs of useful objects.
    /// * `data` - An optional list of data keys to specify additional data to include in the response.
    ///
    /// # Returns
    ///
    /// A `Result` containing detailed information about the Free Company as `FreeCompanyResult` or a `reqwest::Error`
    /// if the request to XIVAPI fails.
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

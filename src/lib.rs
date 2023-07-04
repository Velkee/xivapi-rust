use reqwest::Client;

pub mod character;
pub mod enums;
pub mod freecompany;
pub mod pagination;

use character::{CharacterProfileResult, CharacterSearchResults};
use enums::{ExtraData, Server};

#[cfg(test)]
mod tests {
    use crate::{Server, XIVAPIClient};

    #[tokio::test]
    async fn test_character_search() {
        let client = XIVAPIClient::new();

        client
            .character_search("Tami Pesagniyah".to_string(), Some(Server::Omega), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_character_lookup() {
        let client = XIVAPIClient::new();

        let result = client
            .character_search("Tami Pesagniyah".to_string(), Some(Server::Omega), None)
            .await
            .unwrap();

        client
            .character_lookup(result.results[0].id, false, None)
            .await
            .unwrap();
    }
}

pub struct XIVAPIClient {
    client: Client,
}

impl XIVAPIClient {
    pub fn new() -> Self {
        let client = Client::new();
        XIVAPIClient { client }
    }

    pub async fn character_search(
        &self,
        name: String,
        server: Option<Server>,
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

    pub async fn character_lookup(
        &self,
        character_id: u32,
        extended: bool,
        data: Option<Vec<ExtraData>>,
    ) -> Result<CharacterProfileResult, reqwest::Error> {
        let mut seach_params = Vec::new();

        if extended {
            seach_params.push("extended=1".to_string())
        }

        if let Some(data) = data {
            let data = data
                .iter()
                .map(|data| data.to_string())
                .collect::<Vec<String>>()
                .join(",");

            seach_params.push(format!("data={}", data))
        }

        let search_string = format!(
            "https://xivapi.com/character/{}?{}",
            character_id,
            seach_params.join("&")
        );

        let result: CharacterProfileResult =
            self.client.get(search_string).send().await?.json().await?;

        Ok(result)
    }
}

impl Default for XIVAPIClient {
    fn default() -> Self {
        Self::new()
    }
}

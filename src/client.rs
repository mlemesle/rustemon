//! Defines the client used to access Pokeapi.

use http_cache_reqwest::{CACacheManager, Cache, HttpCache};
use reqwest::{Client, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::de::DeserializeOwned;

use crate::error::Error;

// Reexport to ease overloading.
pub use http_cache_reqwest::{CacheMode, CacheOptions};

/// Custom client used to call Pokeapi.
pub struct RustemonClient {
    client: ClientWithMiddleware,
}

impl RustemonClient {
    /// Returns a new instance of RustemonClient.
    ///
    /// # Arguments
    ///
    /// * `cache_mode` - The [CacheMode] to inject into the client.
    /// * `options` - The [CacheOptions] to inject into the client.
    pub fn new(cache_mode: CacheMode, options: Option<CacheOptions>) -> Self {
        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: cache_mode,
                    manager: CACacheManager::default(),
                    options,
                }))
                .build(),
        }
    }

    /// Returns a new instance of RustemonClient,
    /// while passing an unchecked path String to CACacheManager
    ///
    /// # Arguments
    ///
    /// * `cache_path` - The String representing a path to pass to the cache manager.
    /// * `cache_mode` - The [CacheMode] to inject into the client.
    /// * `options` - The [CacheOptions] to inject into the client.
    pub fn new_path_unchecked(
        cache_path: String,
        cache_mode: CacheMode,
        options: Option<CacheOptions>,
    ) -> Self {
        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: cache_mode,
                    manager: CACacheManager { path: cache_path },
                    options,
                }))
                .build(),
        }
    }

    /// Returns the deserialized answer resulting from the call made to Pokeapi.
    ///
    /// # Arguments
    ///
    /// `url` - The url to call in order to retrieves the json to deserialize.
    pub(crate) async fn get_by_url<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let json_answer = self.client.get(url).send().await?.json().await?;
        Ok(json_answer)
    }
}

impl Default for RustemonClient {
    /// Returns a RustemonClient with default configuration.
    fn default() -> Self {
        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CACacheManager::default(),
                    options: None,
                }))
                .build(),
        }
    }
}

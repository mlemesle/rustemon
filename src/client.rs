//! Defines the client used to access Pokeapi.

use http_cache_reqwest::{Cache, HttpCache};
use reqwest::{Client, IntoUrl, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::de::DeserializeOwned;

use crate::error::Error;

// Reexport to ease overloading.
pub use http_cache_reqwest::{CACacheManager, CacheMode, CacheOptions};

/// Environment to target while calling PokeApi.
pub enum Environment {
    /// Targets the production environment.
    Production,
    /// Targets the stating environment.
    Staging,
    /// Targets a custom environment of PokeApi, a local deployment through Docker for example.
    Custom(Url),
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}

impl From<Environment> for Url {
    fn from(value: Environment) -> Self {
        match value {
            Environment::Production => Url::parse("https://pokeapi.co/api/v2/").unwrap(),
            Environment::Staging => Url::parse("https://staging.pokeapi.co/api/v2/").unwrap(),
            Environment::Custom(url) => url,
        }
    }
}

/// Builder used to ease the configuration of `RustemonClient`.
pub struct RustemonClientBuilder {
    cache: HttpCache<CACacheManager>,
    environment: Environment,
}

impl RustemonClientBuilder {
    /// Creates a new builder, having default configuration.
    pub fn new() -> Self {
        Self {
            cache: HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            },
            environment: Environment::default(),
        }
    }

    /// Configure the CacheMode of the builder. See [CacheMode].
    pub fn with_mode(mut self, cache_mode: CacheMode) -> Self {
        self.cache.mode = cache_mode;
        self
    }

    /// Configure the manager of the builder. See [CACacheManager].
    pub fn with_manager(mut self, manager: CACacheManager) -> Self {
        self.cache.manager = manager;
        self
    }

    /// Configure the cache options of the builder. See [CacheOptions].
    pub fn with_options(mut self, options: CacheOptions) -> Self {
        self.cache.options = Some(options);
        self
    }

    /// Configure the environment of the builder. See [Environment].
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Consumes the builder in order to create a [RustemonClient].
    pub fn build(self) -> RustemonClient {
        RustemonClient {
            client: ClientBuilder::new(Client::new())
                .with(Cache(self.cache))
                .build(),
            base: Url::from(self.environment),
        }
    }
}

/// Custom client used to call Pokeapi.
pub struct RustemonClient {
    client: ClientWithMiddleware,
    base: Url,
}

/// Inner representation of an endpoint's id. Used to ease the api calls.
pub(crate) enum Id<'a> {
    Int(i64),
    Str(&'a str),
}

impl RustemonClient {
    /// Calls the api through the given [Url].
    async fn inner_get<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        Ok(self.client.get(url).send().await?.json().await?)
    }

    /// Make a call through the client to the given `endpoint`.
    pub(crate) async fn get_by_endpoint<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = self
            .base
            .join(endpoint)
            .map_err(|_| Error::UrlParse(format!("{}/{endpoint}", self.base)))?;
        self.inner_get(url).await
    }

    /// Make a call through the client to the given `endpoint`, adding `limit` and `offset` to the query.
    pub(crate) async fn get_with_limit_and_offset<T>(
        &self,
        endpoint: &str,
        limit: i64,
        offset: i64,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut url = self
            .base
            .join(endpoint)
            .map_err(|_| Error::UrlParse(format!("{}/{endpoint}", self.base)))?;
        url.set_query(Some(&format!("limit={limit}&offset={offset}")));
        self.inner_get(url).await
    }

    /// Make a call though the client to the given `endpoint`, targetting a specific resource described by [Id].
    pub(crate) async fn get_by_endpoint_and_id<'a, T>(
        &self,
        endpoint: &str,
        id: Id<'a>,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let inner_id = match id {
            Id::Int(i) => i.to_string(),
            Id::Str(s) => s.to_owned(),
        };
        let url = self
            .base
            .join(&format!("{endpoint}/{inner_id}"))
            .map_err(|_| Error::UrlParse(format!("{}/{endpoint}/{inner_id}", self.base)))?;
        self.inner_get(url).await
    }

    /// Make a call through the client from a given [IntoUrl].
    pub(crate) async fn get_by_url<T>(&self, url: impl IntoUrl) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.inner_get(url.into_url()?).await
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
            base: Url::from(Environment::default()),
        }
    }
}

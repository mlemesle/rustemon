//! Defines the client used to access Pokeapi.

use http_cache_reqwest::{Cache, CacheManager, HttpCache, HttpCacheOptions};
use reqwest::{Client, IntoUrl, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::de::DeserializeOwned;

use crate::error::Error;

// Reexport to ease overloading.
pub use http_cache_reqwest::{CacheMode, CacheOptions};

pub use http_cache_reqwest::{CACacheManager, MokaManager};

/// Environment to target while calling PokeApi.
#[derive(Clone)]
pub enum Environment {
    /// Targets the production environment.
    Production,
    /// Targets the stating environment.
    Staging,
    /// Targets a custom environment of PokeApi, a local deployment through Docker for example.
    Custom(String),
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}

impl TryFrom<Environment> for Url {
    type Error = Error;

    fn try_from(value: Environment) -> Result<Self, Self::Error> {
        match value {
            Environment::Production => Ok(Url::parse("https://pokeapi.co/api/v2/").unwrap()),
            Environment::Staging => Ok(Url::parse("https://staging.pokeapi.co/api/v2/").unwrap()),
            Environment::Custom(mut s) => {
                if !s.ends_with('/') {
                    s.push('/');
                }

                Url::parse(&s).map_err(|_| Error::UrlParse(s))
            }
        }
    }
}

/// Builder used to ease the configuration of `RustemonClient`.
pub struct RustemonClientBuilder<T: CacheManager> {
    cache: HttpCache<T>,
    environment: Environment,
}

impl Default for RustemonClientBuilder<CACacheManager> {
    fn default() -> Self {
        Self {
            cache: HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: HttpCacheOptions::default(),
            },
            environment: Environment::default(),
        }
    }
}

impl Default for RustemonClientBuilder<MokaManager> {
    fn default() -> Self {
        Self {
            cache: HttpCache {
                mode: CacheMode::Default,
                manager: MokaManager::default(),
                options: HttpCacheOptions::default(),
            },
            environment: Environment::default(),
        }
    }
}

impl<T: CacheManager> RustemonClientBuilder<T> {
    /// Configure the CacheMode of the builder. See [CacheMode].
    pub fn with_mode(mut self, cache_mode: CacheMode) -> Self {
        self.cache.mode = cache_mode;
        self
    }

    /// Configure the manager of the builder. See [CacheManager].
    pub fn with_manager(mut self, manager: T) -> Self {
        self.cache.manager = manager;
        self
    }

    /// Configure the cache options of the builder. See [CacheOptions].
    pub fn with_options(mut self, options: CacheOptions) -> Self {
        self.cache.options.cache_options = Some(options);
        self
    }

    /// Configure the environment of the builder. See [Environment].
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Consumes the builder in order to create a [RustemonClient].
    pub fn try_build(self) -> Result<RustemonClient, Error> {
        Ok(RustemonClient {
            client: ClientBuilder::new(Client::new())
                .with(Cache(self.cache))
                .build(),
            base: Url::try_from(self.environment.clone())?,
        })
    }
}

/// Custom client used to call Pokeapi.
#[derive(Debug)]
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
    pub(crate) async fn get_by_endpoint_and_id<T>(
        &self,
        endpoint: &str,
        id: Id<'_>,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let endpoint_id = match id {
            Id::Int(i) => format!("{endpoint}/{i}"),
            Id::Str(s) => format!("{endpoint}/{s}"),
        };
        let url = self
            .base
            .join(&endpoint_id)
            .map_err(|_| Error::UrlParse(format!("{}/{endpoint_id}", self.base)))?;
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
        let manager = CACacheManager::default();

        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager,
                    options: HttpCacheOptions::default(),
                }))
                .build(),
            base: Url::try_from(Environment::default()).unwrap(),
        }
    }
}

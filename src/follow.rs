use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::{
    client::RustemonClient,
    error::Error,
    model::resource::{ApiResource, NamedApiResource},
};

/// Trait representing types that can be followed to ease navigation through the API.
#[async_trait]
pub trait Follow<T>
where
    T: DeserializeOwned + Send + Sync,
{
    /// Returns the resource pointed by the [NamedApiResource]. Follows its inner URL and gives back the result.
    ///
    /// # Arguments
    ///
    /// `rustemon_client` - The [RustemonClient] to use to access the resource.
    async fn follow(&self, rustemon_client: &RustemonClient) -> Result<T, Error>;
}

#[async_trait]
impl<T> Follow<T> for NamedApiResource<T>
where
    T: DeserializeOwned + Send + Sync,
{
    async fn follow(&self, rustemon_client: &RustemonClient) -> Result<T, Error> {
        rustemon_client.get_by_url(&self.url).await
    }
}

#[async_trait]
impl<T> Follow<T> for ApiResource<T>
where
    T: DeserializeOwned + Send + Sync,
{
    async fn follow(&self, rustemon_client: &RustemonClient) -> Result<T, Error> {
        rustemon_client.get_by_url(&self.url).await
    }
}

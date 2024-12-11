use std::future::Future;

use serde::Deserialize;

use crate::{
    client::RustemonClient,
    error::Error,
    model::resource::{ApiResource, NamedApiResource},
};

/// Trait representing types that can be followed to ease navigation through the API.
pub trait Follow<T>
where
    T: for<'a> Deserialize<'a>,
{
    /// Returns the resource pointed by the resource. Follows its inner URL and gives back the result.
    ///
    /// # Arguments
    ///
    /// `rustemon_client` - The [RustemonClient] to use to access the resource.
    fn follow(&self, rustemon_client: &RustemonClient) -> impl Future<Output = Result<T, Error>>;
}

impl<T> Follow<T> for NamedApiResource<T>
where
    T: for<'a> Deserialize<'a>,
{
    async fn follow(&self, rustemon_client: &RustemonClient) -> Result<T, Error> {
        rustemon_client.get_by_url(&self.url).await
    }
}

impl<T> Follow<T> for ApiResource<T>
where
    T: for<'a> Deserialize<'a>,
{
    async fn follow(&self, rustemon_client: &RustemonClient) -> Result<T, Error> {
        rustemon_client.get_by_url(&self.url).await
    }
}

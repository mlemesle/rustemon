macro_rules! endpoint {
    ($type:ty; for $name:literal) => {
        use reqwest::Url;

        use crate::model::resource::{NamedApiResourceList, NamedApiResource};
        use crate::client::RustemonClient;
        use crate::error::Error;

        const ENDPOINT: &str = concat!("https:///pokeapi.co/api/v2/", $name, "/");

        /// Returns the default page regarding the resource.
        ///
        /// # Arguments
        ///
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_page(rustemon_client: &RustemonClient) -> Result<NamedApiResourceList<$type>, Error> {
            let url = Url::parse(ENDPOINT).unwrap();
            rustemon_client.get_by_url::<NamedApiResourceList<$type>>(url).await
        }

        /// Returns the page targeted by the parameters.
        ///
        /// # Arguments
        ///
        /// `offset` - The offset to start retrieving the data from.
        /// `limit` - Maximum number of elements returned by the call.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_page_with_param(
            offset: i64,
            limit: i64,
            rustemon_client: &RustemonClient
        ) -> Result<NamedApiResourceList<$type>, Error> {
            let url = Url::parse_with_params(
                ENDPOINT,
                &[("limit", limit.to_string()), ("offset", offset.to_string())],
            )
            .unwrap();
            rustemon_client.get_by_url::<NamedApiResourceList<$type>>(url).await
        }

        /// Returns all pages from the given resource.
        ///
        /// # Arguments
        ///
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_all_pages(rustemon_client: &RustemonClient) -> Result<Vec<NamedApiResource<$type>>, Error> {
            let url = Url::parse(ENDPOINT).unwrap();

            let mut page = rustemon_client.get_by_url::<NamedApiResourceList<$type>>(url).await?;
            let mut all_pages = Vec::with_capacity(page.count as usize);

            all_pages.append(&mut page.results);

            while let Some(ref next_page_url) = page.next {
                let next_page_url = Url::parse(next_page_url).unwrap();
                page = rustemon_client.get_by_url::<NamedApiResourceList<$type>>(next_page_url).await?;

                all_pages.append(&mut page.results);
            }

            Ok(all_pages)
        }

        /// Returns the resource, using its id.
        ///
        /// # Arguments
        ///
        /// `id` - The unique ID of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$type, Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(&id.to_string()).unwrap();
            rustemon_client.get_by_url::<$type>(url).await
        }

        /// Returns the resource, using its name.
        ///
        /// # Arguments
        ///
        /// `name` - The name of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_name(name: &str, rustemon_client: &RustemonClient) -> Result<$type, Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(name).unwrap();
            rustemon_client.get_by_url::<$type>(url).await
        }
    };

    ($type:ty; for $name:literal; with $(($sub:ident, $sub_type:ty))+) => {

        crate::endpoint!($type; for $name);

        $(
            pub mod $sub {

                use reqwest::Url;
                use crate::client::RustemonClient;
                use crate::error::Error;

                use super::ENDPOINT;

                const SUB_ENDPOINT: &str = stringify!($sub);

                /// Returns the resource, using its id.
                ///
                /// # Arguments
                ///
                /// `id` - The unique ID of the resource to get.
                /// `rustemon_client` - The [RustemonClient] to use to access the resource.
                pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$sub_type, Error> {
                    let sub_path = format!("{}/{}", id, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    rustemon_client.get_by_url::<$sub_type>(url).await
                }

                /// Returns the resource, using its name.
                ///
                /// # Arguments
                ///
                /// `name` - The name of the resource to get.
                /// `rustemon_client` - The [RustemonClient] to use to access the resource.
                pub async fn get_by_name(name: &'static str, rustemon_client: &RustemonClient) -> Result<$sub_type, Error> {
                    let sub_path = format!("{}/{}", name, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    rustemon_client.get_by_url::<$sub_type>(url).await
                }
            }
        )+
    };
}

pub(crate) use endpoint;

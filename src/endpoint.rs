macro_rules! endpoint {
    ($type:ty; for $name:literal) => {
        use reqwest::Url;

        use crate::model::resource::NamedApiResourceList;
        use crate::client::RustemonClient;

        const ENDPOINT: &str = concat!("https:///pokeapi.co/api/v2/", $name, "/");

        /// Returns the default page regarding the resource.
        pub async fn get_page(rustemon_client: &RustemonClient) -> Result<NamedApiResourceList<$type>, anyhow::Error> {
            let url = Url::parse(ENDPOINT).unwrap();
            rustemon_client.get_by_url::<NamedApiResourceList<$type>>(url).await
        }

        /// Returns the page targeted by the parameters.
        ///
        /// # Arguments
        ///
        /// `offset` - The offset to start retrieving the data from.
        /// `limit` - Maximum number of elements returned by the call.
        pub async fn get_page_with_param(
            offset: i64,
            limit: i64,
            rustemon_client: &RustemonClient
        ) -> Result<NamedApiResourceList<$type>, anyhow::Error> {
            let url = Url::parse_with_params(
                ENDPOINT,
                &[("limit", limit.to_string()), ("offset", offset.to_string())],
            )
            .unwrap();
            rustemon_client.get_by_url::<NamedApiResourceList<$type>>(url).await
        }

        /// Returns the resource, using its id.
        ///
        /// # Arguments
        ///
        /// `id` - The unique ID of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$type, anyhow::Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(&id.to_string()).unwrap();
            rustemon_client.get_by_url::<$type>(url).await
        }

        /// Returns the resource, using its name.
        ///
        /// # Arguments
        ///
        /// `name` - The name of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_name(name: &str, rustemon_client: &RustemonClient) -> Result<$type, anyhow::Error> {
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

                use super::ENDPOINT;

                const SUB_ENDPOINT: &str = stringify!($sub);

                /// Returns the resource, using its id.
                ///
                /// # Arguments
                ///
                /// `id` - The unique ID of the resource to get.
                /// `rustemon_client` - The [RustemonClient] to use to access the resource.
                pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$sub_type, anyhow::Error> {
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
                pub async fn get_by_name(name: &'static str, rustemon_client: &RustemonClient) -> Result<$sub_type, anyhow::Error> {
                    let sub_path = format!("{}/{}", name, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    rustemon_client.get_by_url::<$sub_type>(url).await
                }
            }
        )+
    };
}

pub(crate) use endpoint;

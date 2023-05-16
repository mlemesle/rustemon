macro_rules! endpoint {
    ($type:ty; for $name:literal) => {
        use crate::model::resource::{NamedApiResourceList, NamedApiResource};
        use crate::client::{RustemonClient, Id};
        use crate::error::Error;

        /// Returns the default page regarding the resource.
        ///
        /// # Arguments
        ///
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_page(rustemon_client: &RustemonClient) -> Result<NamedApiResourceList<$type>, Error> {
            rustemon_client.get_by_endpoint::<NamedApiResourceList<$type>>($name).await
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
            rustemon_client.get_with_limit_and_offset::<NamedApiResourceList<$type>>($name, limit, offset).await
        }

        /// Returns all entries from the given resource.
        ///
        /// # Arguments
        ///
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_all_entries(rustemon_client: &RustemonClient) -> Result<Vec<NamedApiResource<$type>>, Error> {
            let mut first_page = get_page(rustemon_client).await?;
            let first_page_entries_count = first_page.results.len() as i64;

            let total_entries_count = first_page.count;
            let other_entries_count = total_entries_count - first_page_entries_count;

            let mut all_entries = Vec::with_capacity(total_entries_count as usize);
            all_entries.append(&mut first_page.results);

            let mut other_entries = get_page_with_param(first_page_entries_count, other_entries_count, &rustemon_client).await?;
            all_entries.append(&mut other_entries.results);

            Ok(all_entries)
        }

        /// Returns the resource, using its id.
        ///
        /// # Arguments
        ///
        /// `id` - The unique ID of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$type, Error> {
            rustemon_client.get_by_endpoint_and_id::<$type>($name, Id::Int(id)).await
        }

        /// Returns the resource, using its name.
        ///
        /// # Arguments
        ///
        /// `name` - The name of the resource to get.
        /// `rustemon_client` - The [RustemonClient] to use to access the resource.
        pub async fn get_by_name(name: &str, rustemon_client: &RustemonClient) -> Result<$type, Error> {
            rustemon_client.get_by_endpoint_and_id::<$type>($name, Id::Str(name)).await
        }
    };

    ($type:ty; for $name:literal; with $(($sub:ident, $sub_type:ty))+) => {

        crate::endpoint!($type; for $name);

        $(
            /// Give access to the sub endpoint.
            pub mod $sub {

                use super::RustemonClient;
                use super::Error;

                const SUB_STR: &'static str = stringify!($sub);

                /// Returns the resource, using its id.
                ///
                /// # Arguments
                ///
                /// `id` - The unique ID of the resource to get.
                /// `rustemon_client` - The [RustemonClient] to use to access the resource.
                pub async fn get_by_id(id: i64, rustemon_client: &RustemonClient) -> Result<$sub_type, Error> {
                    let sub_path = format!("{}/{}/{}", $name, id, SUB_STR);
                    rustemon_client.get_by_endpoint::<$sub_type>(&sub_path).await
                }

                /// Returns the resource, using its name.
                ///
                /// # Arguments
                ///
                /// `name` - The name of the resource to get.
                /// `rustemon_client` - The [RustemonClient] to use to access the resource.
                pub async fn get_by_name(name: &'static str, rustemon_client: &RustemonClient) -> Result<$sub_type, Error> {
                    let sub_path = format!("{}/{}/{}", $name, name, SUB_STR);
                    rustemon_client.get_by_endpoint::<$sub_type>(&sub_path).await
                }
            }
        )+
    };
}

pub(crate) use endpoint;

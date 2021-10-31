macro_rules! endpoint {
    ($type:ty; for $name:literal) => {
        use cached::proc_macro::cached;
        use reqwest::Url;

        use crate::api::model::resource::NamedApiResourceList;

        const ENDPOINT: &str = concat!("https:///pokeapi.co/api/v2/", $name, "/");

        #[cached(size = 1, result = true)]
        pub fn get_page() -> Result<NamedApiResourceList, reqwest::Error> {
            reqwest::blocking::get(ENDPOINT)?.json::<NamedApiResourceList>()
        }

        pub fn get_page_with_param(
            offset: i64,
            limit: i64,
        ) -> Result<NamedApiResourceList, reqwest::Error> {
            let url = Url::parse_with_params(
                ENDPOINT,
                &[("limit", limit.to_string()), ("offset", offset.to_string())],
            )
            .unwrap();
            reqwest::blocking::get(url)?.json::<NamedApiResourceList>()
        }

        #[cached(result = true)]
        pub fn get_by_id(id: i64) -> Result<$type, reqwest::Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(&id.to_string()).unwrap();
            reqwest::blocking::get(url)?.json::<$type>()
        }

        #[cached(result = true)]
        pub fn get_by_name(name: &'static str) -> Result<$type, reqwest::Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(name).unwrap();
            reqwest::blocking::get(url)?.json::<$type>()
        }
    };

    ($type:ty; for $name:literal; with $(($sub:ident, $sub_type:ty))+) => {

        crate::api::endpoint::endpoint::endpoint!($type; for $name);

        $(
            pub mod $sub {

                use cached::proc_macro::cached;
                use reqwest::Url;

                use super::ENDPOINT;

                const SUB_ENDPOINT: &str = stringify!($sub);

                #[cached(result = true)]
                pub fn get_by_id(id: i64) -> Result<$sub_type, reqwest::Error> {
                    let sub_path = format!("{}/{}", id, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    reqwest::blocking::get(url)?.json::<$sub_type>()
                }

                #[cached(result = true)]
                pub fn get_by_name(name: &'static str) -> Result<$sub_type, reqwest::Error> {
                    let sub_path = format!("{}/{}", name, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    println!("{}", url);
                    reqwest::blocking::get(url)?.json::<$sub_type>()
                }
            }
        )+
    };
}

pub(crate) use endpoint;
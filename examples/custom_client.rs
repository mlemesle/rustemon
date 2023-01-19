use std::time::Duration;

use http_cache_reqwest::{CACacheManager, CacheMode, CacheOptions};
use rustemon::client::{Environment, RustemonClientBuilder};

#[tokio::main]
async fn main() {
    let rustemon_client = RustemonClientBuilder::default()
        .with_mode(CacheMode::NoStore)
        .with_manager(CACacheManager::default())
        .with_options(CacheOptions {
            shared: true,
            cache_heuristic: 0.2,
            immutable_min_time_to_live: Duration::from_secs(3600),
            ignore_cargo_cult: true,
        })
        .with_environment(Environment::Custom(
            "https://pokeapi.co/api/v2/".parse().unwrap(),
        ))
        .try_build()
        .unwrap();

    let location_area = rustemon::locations::location_area::get_by_id(296, &rustemon_client)
        .await
        .unwrap();

    println!("{location_area:#?}");
}

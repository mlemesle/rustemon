#[tokio::main]
async fn main() {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = rustemon::pokemon::pokemon::get_by_name("charizard", &rustemon_client).await.unwrap();
    let toml_pokemon = toml::to_string(&pokemon).unwrap();
    println!("{toml_pokemon}");
}

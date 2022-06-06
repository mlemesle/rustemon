#[tokio::main]
async fn main() {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = rustemon::pokemon::pokemon::get_by_name("charizard", &rustemon_client).await;
    let species_resource = pokemon.unwrap().species.unwrap();
    let species = species_resource.follow(&rustemon_client).await;
    println!("{:#?}", species);
}

use rustemon::Follow;

#[tokio::main]
async fn main() {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = rustemon::pokemon::pokemon::get_by_name("charizard", &rustemon_client).await;
    let species_resource = pokemon.unwrap().species;
    let species = species_resource.follow(&rustemon_client).await;
    println!("{:#?}", species);

    let evolution_chain = species
        .unwrap()
        .evolution_chain
        .follow(&rustemon_client)
        .await;
    println!("{:#?}", evolution_chain);
}

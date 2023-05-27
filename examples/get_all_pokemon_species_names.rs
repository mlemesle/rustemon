#[tokio::main]
async fn main() {
    // Gets the names of all pokémon types.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let species = rustemon::pokemon::pokemon_species::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let species_names: Vec<String> = species.into_iter().map(|species| species.name).collect();

    println!("All Pokémon species names: {:?}", species_names);
}

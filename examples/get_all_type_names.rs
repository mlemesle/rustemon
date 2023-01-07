#[tokio::main]
async fn main() {
    // Gets the names of all pokmon types. Retrieving them 5 by 5 from the api.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let types = rustemon::pokemon::type_::get_all_pages(&rustemon_client)
        .await
        .unwrap();
    let type_names: Vec<String> = types.into_iter().map(|type_| type_.name).collect();

    println!("All Pokémon type names : {:#?}", type_names);
}

#[tokio::main]
async fn main() {
    // Gets the names of all pokmon types. Retrieving them 5 by 5 from the api.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let mut offset = 0;
    let limit = 5;
    let number_of_type = rustemon::pokemon::type_::get_page(&rustemon_client)
        .await
        .unwrap()
        .count;
    let mut type_names: Vec<String> = vec![];
    while offset < number_of_type {
        match rustemon::pokemon::type_::get_page_with_param(offset, limit, &rustemon_client).await {
            Ok(page) => {
                for name in page.results {
                    type_names.push(name.name);
                }
                offset += limit;
            }
            Err(err) => {
                println!("Error occured : {}", err);
                break;
            }
        }
    }

    println!("All Pokémon type names : {:#?}", type_names);
}

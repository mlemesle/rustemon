#[tokio::main]
async fn main() {
    // Gets a paginated sets of pokemon, starting from 12 and up to 5 elements in the response.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon_page =
        rustemon::pokemon::pokemon::get_page_with_param(12, 5, &rustemon_client).await;
    match pokemon_page {
        Ok(page) => println!("{:#?}", page),
        Err(err) => println!("The following error occured : {}", err),
    }
}

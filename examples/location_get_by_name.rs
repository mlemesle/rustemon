#[tokio::main]
async fn main() {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let location =
        rustemon::locations::location::get_by_name("cerulean-city", &rustemon_client).await;
    match location {
        Ok(l) => println!("{:#?}", l),
        Err(err) => println!("An error occured : {}", err),
    }
}

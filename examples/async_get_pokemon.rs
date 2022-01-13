#[tokio::main]
async fn main() {
    let pokemon = rustemon::pokemon::pokemon::get_by_name("charizard").await;
    println!("{:#?}", pokemon);
}

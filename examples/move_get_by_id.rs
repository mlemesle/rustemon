#[tokio::main]
async fn main() {
    // Gets a move by its id.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let move_ = rustemon::moves::move_::get_by_id(65, &rustemon_client).await;
    match move_ {
        Ok(m) => println!("{:#?}", m),
        Err(err) => println!("An error occured : {}", err),
    }
}

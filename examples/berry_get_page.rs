#[tokio::main]
async fn main() {
    // Gets the very first paginated result for the berry endpoint.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let berry_page = rustemon::berries::berry::get_page(&rustemon_client).await;
    match berry_page {
        Ok(page) => println!("{page:#?}"),
        Err(err) => println!("The following error occured : {err}"),
    }
}

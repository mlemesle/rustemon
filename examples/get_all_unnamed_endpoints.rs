#[tokio::main]
async fn main() {
    // Gets all entries from all unnamed endpoints.
    let rustemon_client = rustemon::client::RustemonClient::default();

    rustemon::pokemon::characteristic::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    rustemon::pokemon::characteristic::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    rustemon::evolution::evolution_chain::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    rustemon::evolution::evolution_chain::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    rustemon::machines::machine::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    rustemon::machines::machine::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    rustemon::contests::contest_effect::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    rustemon::contests::contest_effect::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    rustemon::contests::super_contest_effect::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    rustemon::contests::super_contest_effect::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
}

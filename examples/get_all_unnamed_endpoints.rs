#[tokio::main]
async fn main() {
    // Gets all entries from all unnamed endpoints.
    let rustemon_client = rustemon::client::RustemonClient::default();
    let characteristics = rustemon::pokemon::characteristic::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let characteristic = rustemon::pokemon::characteristic::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    let evolution_chains = rustemon::evolution::evolution_chain::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let evolution_chain = rustemon::evolution::evolution_chain::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    let machines = rustemon::machines::machine::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let machine = rustemon::machines::machine::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    let contest_effects = rustemon::contests::contest_effect::get_all_entries(&rustemon_client)
        .await
        .unwrap();
    let contest_effect = rustemon::contests::contest_effect::get_by_id(1, &rustemon_client)
        .await
        .unwrap();
    let super_contest_effects =
        rustemon::contests::super_contest_effect::get_all_entries(&rustemon_client)
            .await
            .unwrap();
    let super_contest_effect =
        rustemon::contests::super_contest_effect::get_by_id(1, &rustemon_client)
            .await
            .unwrap();
}

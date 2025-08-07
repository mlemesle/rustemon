use std::{any, fs::OpenOptions, io::BufWriter};

use anyhow::anyhow;
use rustemon::{
    berries, client::RustemonClient, contests, encounters, evolution, games, items, locations,
    machines, moves, pokemon, utility,
};
use serde::Serialize;
use tera::{Context, Tera};

async fn get_json_body<T: Serialize, Fut: Future<Output = Result<T, rustemon::error::Error>>>(
    supplier: impl FnOnce() -> Fut,
) -> anyhow::Result<(String, String, String)> {
    let value = supplier().await?;
    let json_body = serde_json::to_string(&value)?;

    let full_path = any::type_name::<T>();
    let type_name = full_path
        .rsplit_once("::")
        .ok_or_else(|| anyhow!("No type name for {full_path}"))?
        .1;
    let mut snake_case_name = String::new();
    for c in type_name.chars() {
        if c.is_ascii_uppercase() {
            snake_case_name.push('_');
            snake_case_name.push(c.to_ascii_lowercase());
        } else {
            snake_case_name.push(c);
        }
    }

    let crate_path = full_path.replace("rustemon", "crate");

    Ok((snake_case_name, crate_path, json_body))
}

async fn get_data(rc: &RustemonClient) -> anyhow::Result<Vec<(String, String, String)>> {
    let payloads = vec![
        // Berries endpoints
        get_json_body(|| berries::berry::get_by_id(1, rc)).await?,
        get_json_body(|| berries::berry_firmness::get_by_id(1, rc)).await?,
        get_json_body(|| berries::berry_flavor::get_by_id(1, rc)).await?,
        // Contests endpoints
        get_json_body(|| contests::contest_type::get_by_id(1, rc)).await?,
        get_json_body(|| contests::contest_effect::get_by_id(1, rc)).await?,
        get_json_body(|| contests::super_contest_effect::get_by_id(1, rc)).await?,
        // Encounters endpoints
        get_json_body(|| encounters::encounter_method::get_by_id(1, rc)).await?,
        get_json_body(|| encounters::encounter_condition::get_by_id(1, rc)).await?,
        get_json_body(|| encounters::encounter_condition_value::get_by_id(1, rc)).await?,
        // Evolution endpoints
        get_json_body(|| evolution::evolution_chain::get_by_id(2, rc)).await?,
        get_json_body(|| evolution::evolution_trigger::get_by_id(1, rc)).await?,
        // Games endpoints
        get_json_body(|| games::generation::get_by_id(1, rc)).await?,
        get_json_body(|| games::pokedex::get_by_id(2, rc)).await?,
        get_json_body(|| games::version::get_by_id(1, rc)).await?,
        get_json_body(|| games::version_group::get_by_id(1, rc)).await?,
        // Items endpoints
        get_json_body(|| items::item::get_by_id(1, rc)).await?,
        get_json_body(|| items::item_attribute::get_by_id(1, rc)).await?,
        get_json_body(|| items::item_category::get_by_id(3, rc)).await?,
        get_json_body(|| items::item_fling_effect::get_by_id(3, rc)).await?,
        get_json_body(|| items::item_pocket::get_by_id(3, rc)).await?,
        // Locations endpoints
        get_json_body(|| locations::location::get_by_id(5, rc)).await?,
        get_json_body(|| locations::location_area::get_by_id(5, rc)).await?,
        get_json_body(|| locations::pal_park_area::get_by_id(1, rc)).await?,
        get_json_body(|| locations::region::get_by_id(1, rc)).await?,
        // Machines endpoints
        get_json_body(|| machines::machine::get_by_id(1, rc)).await?,
        // Moves endpoints
        get_json_body(|| moves::move_::get_by_id(126, rc)).await?,
        get_json_body(|| moves::move_ailment::get_by_id(2, rc)).await?,
        get_json_body(|| moves::move_battle_style::get_by_id(1, rc)).await?,
        get_json_body(|| moves::move_category::get_by_id(0, rc)).await?,
        get_json_body(|| moves::move_damage_class::get_by_id(2, rc)).await?,
        get_json_body(|| moves::move_learn_method::get_by_id(1, rc)).await?,
        get_json_body(|| moves::move_target::get_by_id(1, rc)).await?,
        // Pokemon endpoints
        get_json_body(|| pokemon::ability::get_by_id(9, rc)).await?,
        get_json_body(|| pokemon::characteristic::get_by_id(1, rc)).await?,
        get_json_body(|| pokemon::egg_group::get_by_id(14, rc)).await?,
        get_json_body(|| pokemon::gender::get_by_id(3, rc)).await?,
        get_json_body(|| pokemon::growth_rate::get_by_id(1, rc)).await?,
        get_json_body(|| pokemon::nature::get_by_id(2, rc)).await?,
        get_json_body(|| pokemon::pokeathlon_stat::get_by_id(2, rc)).await?,
        get_json_body(|| pokemon::pokemon::get_by_id(6, rc)).await?,
        get_json_body(|| pokemon::pokemon_color::get_by_id(5, rc)).await?,
        get_json_body(|| pokemon::pokemon_form::get_by_id(6, rc)).await?,
        get_json_body(|| pokemon::pokemon_habitat::get_by_id(4, rc)).await?,
        get_json_body(|| pokemon::pokemon_shape::get_by_id(9, rc)).await?,
        get_json_body(|| pokemon::pokemon_species::get_by_id(6, rc)).await?,
        get_json_body(|| pokemon::stat::get_by_id(4, rc)).await?,
        get_json_body(|| pokemon::type_::get_by_id(10, rc)).await?,
        // Utility endpoints
        get_json_body(|| utility::language::get_by_id(2, rc)).await?,
    ];

    Ok(payloads)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rc = RustemonClient::default();

    let mut template = Tera::default();
    template.add_template_file(
        "./rustemon-static-test/src/template.tera",
        Some("template.tera"),
    )?;

    let payloads = get_data(&rc).await?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("./rustemon/src/static_resources.rs")?;
    let writer = BufWriter::new(file);

    let mut context = Context::new();
    context.insert("payloads", &payloads);

    template.render_to("template.tera", &context, writer)?;

    Ok(())
}

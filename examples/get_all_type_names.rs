extern crate rustemon;

fn main() {
    // Gets the names of all pokmon types. Retrieving them 5 by 5 from the api.
    let mut offset = 0;
    let limit = 5;
    let number_of_type = rustemon::blocking::pokemon::type_::get_page()
        .unwrap()
        .count
        .unwrap();
    let mut type_names: Vec<String> = vec![];
    while offset < number_of_type {
        match rustemon::blocking::pokemon::type_::get_page_with_param(offset, limit) {
            Ok(page) => {
                for name in page.results.unwrap() {
                    type_names.push(name.name.unwrap());
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

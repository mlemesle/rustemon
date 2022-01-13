extern crate rustemon;

fn main() {
    let location = rustemon::blocking::locations::location::get_by_name("cerulean-city");
    match location {
        Ok(l) => println!("{:#?}", l),
        Err(err) => println!("An error occured : {}", err),
    }
}

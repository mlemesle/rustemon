extern crate rustemon;

fn main() {
    // Gets the very first paginated result for the berry endpoint.
    let berry_page = rustemon::blocking::berries::berry::get_page();
    match berry_page {
        Ok(page) => println!("{:#?}", page),
        Err(err) => println!("The following error occured : {}", err),
    }
}

extern crate rustemon;

fn main() {
    // Gets a move by its id.
    let move_ = rustemon::api::endpoint::moves::move_::get_by_id(65);
    match move_ {
        Ok(m) => println!("{:?}", m),
        Err(err) => println!("An error occured : {}", err),
    }
}

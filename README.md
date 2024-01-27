# Rustemon [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/rustemon.svg
[crates.io]: https://crates.io/crates/rustemon

This library is a wrapper for the awesome [PokeApi](https://pokeapi.co), which provides all the informations you need about Pokémons !

Fully written in Rust, this library covers the whole PokeApi REST api v2.

### How to import it ?

Adds the following in the dependencies of your Cargo.toml :

```toml
rustemon = "3.3.0" 
```

### How to use it ?

As you noticed in [the documentation of PokeApi](https://pokeapi.co/docs/v2), every endpoints and models are declared in groups.

The library kept this group oriented architecture !

### Features

This crate provides two features to enable different cache mechanisms :
* `filesystem-cache` -> A configurable cache stored in files.
* `in-memory-cache` -> A configurable in-memory cache.

`filesystem-cache` is enabled by default.
Be careful, those features are mutually exclusive !!
If you want to use the `in-memory-cache` feature, you can proceed as follow:

```toml
[dependencies]
rustemon = { version = "*", default-features = false, features = ["in-memory-cache"] } 
```

##### Models

All the models are located into the following module :

```rust
rustemon::model
```

For example, if you want the type `Pokemon` located in the group `pokemon`, it is located here :

```rust
rustemon::model::pokemon::Pokemon
```

##### Endpoints

The endpoints follow the same naming rule, for example, if you want to call the evolution chain 
endpoint from the evolution group :

```rust
rustemon::evolution::evolution_chain
```

For each endpoints, 4 functions are defined :

* get_page : without parameters, calls the endpoint without any informations to retrieve the first page of the paginated response for the endpoint, as defined [here](https://pokeapi.co/docs/v2#resource-listspagination-section),
* get_page_with_param : allows you to modify the parameters for the paginated query,
* get_by_id : calls the endpoint using the id of the targeted resource,
* get_by_name : calls the endpoint using the name of the targeted resource.

The endpoints are all async ! Which means you NEED to add a async runtime (such as tokio for example), in order for 
the library to work.

### Caching

All calls to the API are cached by a middleware attached to the [RustemonClient](/src/client.rs) you need to instanciate in order
to make calls to the PokeAPI.

### Examples

Examples and use cases are available in the `examples` folder. For instance, you can run the `async_follow` example using 

```bash
cargo run --example async_follow
```

### License

The license of the library can be found [here](/LICENSE).
  
### Contribution

If you find any bug or improvement that you find valuable, please fill an issue right there -> [project issues](https://github.com/mlemesle/rustemon/issues)

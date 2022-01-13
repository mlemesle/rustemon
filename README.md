# Rustemon

This library is a wrapper for the awesome [PokeApi](https://pokeapi.co), which provides all the informations you need about Pokémons !

Fully written in Rust, this library covers the whole PokeApi REST api v2.

### How to import it ?

Adds the following in the dependencies of your Cargo.toml :

```toml
rustemon = "1.0.0"
```

### How to use it ?

As you noticed in [the documentation of PokeApi](https://pokeapi.co/docs/v2), every endpoints and models are declared in groups.

The library kept this group oriented architecture !

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

By default, the endpoints are all async ! Which means you NEED to add a async runtime (such as tokio for example), in order for 
the library to work.

If you want a synchronous call to the PokeAPI, the `blocking` module is there for you.
The same 4 functions are also available.
If you intend to use the evolution_chain endpoint from the evolution group in a synchronous call :

```rust
rustemon::blocking::evolution::evolution_chain
```

### Caching

All the endpoints are cached automatically when the api's response is a success.
The cache lives for 1 day. After that, the next successful call will refresh the cache for another day.

### License

The license of the library can be found [here](LICENSE).
  
### Contribution

If you find any bug or improvement that you find valuable, please fill an issue right there -> [project issues](https://github.com/mlemesle/rustemon/issues)
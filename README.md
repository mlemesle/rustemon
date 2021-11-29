# Rustemon

This library is a wrapper for the awesome [PokeApi](https://pokeapi.co), which provides all the informations you need about Pokémons !

Fully written in Rust, this library covers the whole PokeApi REST api v2.

### How to import it ?

Adds the following in the dependencies of your Cargo.toml :

```toml
rustemon = "0.1.1"
```

### How to use it ?

As you noticed in [the documentation of PokeApi](https://pokeapi.co/docs/v2), every endpoints and models are declared in groups.

The library kept this group oriented architecture !

##### Models

All the models are located into the following module :

```rust
rustemon::api::model
```

For example, if you want the type `Pokemon` located in the group `pokemon`, it is located here :

```rust
rustemon::api::model::pokemon::Pokemon
```

##### Endpoints

The endpoints follow the same naming rule and can be found in the following module :

```rust
rustemon::api::endpoint
```

For example, if you want to call the evolution chain endpoint from the evolution group :

```rust
rustemon::api::endpoint::evolution::evolution_chain
```


For each endpoints, 4 call are defined :

* get_page : without parameters, calls the endpoint without any informations to retrieve the first page of the paginated response for the endpoint. As defined [here](https://pokeapi.co/docs/v2#resource-listspagination-section),
* get_page_with_param : allows you to modify the parameters for the paginated query,
* get_by_id : calls the endpoint using the id of the targeted resource,
* get_by_name : calls the endpoint using the name of the targeted resource.

All the endpoints are cached automatically when the api's response is a success.

Every endpoints are genereted by a custom macro that you will find [here](src/api/endpoint/endpoint.rs).


### License

The license of the library can be found [here](LICENSE).

### Further improvements

* Make the endpoints async
  
### Contribution

If you find any bug or improvement that you find valuable, please fill an issue right there -> [project issues](https://github.com/mlemesle/rustemon/issues)
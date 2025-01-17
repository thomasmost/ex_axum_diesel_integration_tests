# Mimimum Reproducible Example

- Install the Diesel CLI with `cargo install diesel_cli`
- Initialize the database with `diesel setup`
- Run the server with `cargo run`
- Run the tests with `cargo test`

Tests fail with

```
191 |     pub fn new<A>(app: A) -> Result<Self>
    |            --- required by a bound in this associated function
192 |     where
193 |         A: IntoTransportLayer,
    |            ^^^^^^^^^^^^^^^^^^ required by this bound in `TestServer::new`
```

The failing test can be found [here](https://github.com/thomasmost/ex_axum_diesel_integration_tests/blob/main/integration_tests/src/api_tests/registration.rs)

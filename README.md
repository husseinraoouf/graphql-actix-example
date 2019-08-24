# Graphql Actix Example

A compelete example of graphql api built with:

- [Actix](https://github.com/actix/actix-web) (web server)
- [Diesel](http://diesel.rs) (database)
- [r2d2](https://github.com/sfackler/r2d2) (database connection pool)
- [Juniper](https://github.com/graphql-rust/juniper) (graphql)
- [juniper-from-schema](https://github.com/davidpdrsn/juniper-from-schema) (graphql code generation)

## Running the app

### install needed packages

```bash
# if ubuntu : sudo apt-get install libpq-dev
# if fedora : sudo dnf install postgresql-devel
# if solus : sudo eopkg install postgresql-devel
cargo install diesel_cli --no-default-features --features postgres
```

### init database postgres

```bash
createdb graphql-actix-example
diesel migration run
```

### server

```bash
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:3000
```

Then go to <http://localhost:3000/>.

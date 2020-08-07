Playground Rocket
===
A little playground project to get familiar writing web services in Rust. Uses:

- Rocket as web framework
- Diesel as ORM
- SQLite as database
- Prometheus for monitoring
- Swagger UI using Rocket Okapi

The application serves some data of the USDA food database.

Running
---
Just do a simple cargo run.


Installation
---
Build tools:

sudo apt-get install postgresql-dev
sudo apt-get install libpq-dev

cargo install diesel_cli --no-default-features --features "postgres sqlite"

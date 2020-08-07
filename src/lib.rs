#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod cors;
pub mod data;
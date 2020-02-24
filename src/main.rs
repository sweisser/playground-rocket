#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
// #![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate prometheus;

use rocket::http::RawStr;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use playground_rocket::models::{Food, FoodGroup, JoinResult, JoinResult2};
use prometheus::IntCounter;
use rocket_prometheus::PrometheusMetrics;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use playground_rocket::cors::CorsFairing;

#[database("usda")]
struct USDADbConn(diesel::SqliteConnection);

static ALLFOODS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("allfoods_counter", "allfoods_counter")
        .expect("Could not create lazy IntCounter")
});

static ALLFOODGROUPS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("allfoodgroups_counter", "allfoodgroups_counter")
        .expect("Could not create lazy IntCounter")
});

static INDEX_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("index_counter", "index_counter")
        .expect("Could not create lazy IntCounter")
});

#[get("/")]
fn index() -> &'static str {
    INDEX_COUNTER.inc();

    "Ready!"
}

#[get("/food")]
fn get_all_foods(conn: USDADbConn) -> Json<Vec<Food>> {
    ALLFOODS_COUNTER.inc();

    Json(Food::all(&*conn))
}

#[get("/food/<food_id>/nutrients")]
fn get_food_nutrients_by_id(food_id: i32, conn: USDADbConn) -> Json<Vec<JoinResult2>> {
    Json(Food::get_nutrients(&*conn))
}

#[get("/food/<food_id>")]
fn get_food_by_id(food_id: i32, conn: USDADbConn) -> Json<Food> {
    Json(Food::get_by_id(&*conn, food_id).unwrap())
}

#[get("/food?<search_string>")]
fn search_food(search_string: &RawStr, conn: USDADbConn) -> Json<Vec<Food>> {
    let search = format!("%{}%", search_string.as_str());
    let t1 = Food::search(&*conn, search.to_string());
    return Json(t1);
}

#[get("/foodgroup")]
fn get_all_foodgroups(conn: USDADbConn) -> Json<Vec<FoodGroup>> {
    ALLFOODS_COUNTER.inc();

    Json(FoodGroup::all(&*conn))
}

#[get("/foodgroup/<foodgroup_id>")]
fn get_foodgroup_by_id(foodgroup_id: i32, conn: USDADbConn) -> Json<FoodGroup> {
    Json(FoodGroup::get_by_id(&*conn, foodgroup_id).unwrap())
}

#[get("/jointest")]
fn get_joined_food_groups(conn: USDADbConn) -> Json<Vec<JoinResult>> {
    Json(FoodGroup::all_foods_in_foodgroup(&*conn))
}

#[get("/ip")]
fn ip_man(remote_addr: SocketAddr) -> String {
    format!("Remote Address: {}", remote_addr.ip())
}


fn main() {
    let prometheus = PrometheusMetrics::new();

    prometheus.registry()
        .register(Box::new(ALLFOODS_COUNTER.clone()))
        .unwrap();

    prometheus.registry()
        .register(Box::new(ALLFOODGROUPS_COUNTER.clone()))
        .unwrap();

    prometheus.registry()
        .register(Box::new(INDEX_COUNTER.clone()))
        .unwrap();

    rocket::ignite()
        .mount("/", routes![index,
            get_all_foods, get_food_by_id, get_food_nutrients_by_id,
            search_food,
            get_all_foodgroups, get_foodgroup_by_id,
            get_joined_food_groups,
            ip_man
            ])
        .mount("/metrics", prometheus)
        .attach(USDADbConn::fairing())
        .attach(CorsFairing)
        .launch();
}

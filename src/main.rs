#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
// #![plugin(rocket_codegen)]

extern crate prometheus;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;
use std::net::SocketAddr;

use diesel::{Connection, SqliteConnection};
use prometheus::IntCounter;
use rocket::State;
use rocket::http::RawStr;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_prometheus::PrometheusMetrics;

use playground_rocket::cors::CorsFairing;
use playground_rocket::models::{Food, FoodGroup, JoinResult, JoinResult2};
use playground_rocket::data::{FoodNutrients, map_to_food_nutrients};

#[database("usda")]
struct USDADbConn(diesel::SqliteConnection);

// DONE Split prometheus counters and cache
struct PrometheusState {
    allfoods_counter: IntCounter,
    allfoodgroups_counter: IntCounter,
    nutrients_counter: IntCounter,
    index_counter: IntCounter
}

impl PrometheusState {
    fn new(prometheus: PrometheusMetrics) -> (PrometheusMetrics, PrometheusState) {
        // Create new instance
        let instance = PrometheusState {
            index_counter: IntCounter::new("index_counter", "index_counter")
                .expect("Could not create IntCounter"),
            allfoods_counter: IntCounter::new("allfoods_counter", "allfoods_counter")
                .expect("Could not create IntCounter"),
            allfoodgroups_counter: IntCounter::new("allfoodgroups_counter", "allfoodgroups_counter")
                .expect("Could not create IntCounter"),
            nutrients_counter: IntCounter::new("nutrients_counter", "nutrients_counter")
                .expect("Could not create IntCounter"),
        };

        prometheus.registry()
            .register(Box::new(instance.allfoods_counter.clone()))
            .unwrap();
        prometheus.registry()
            .register(Box::new(instance.allfoodgroups_counter.clone()))
            .unwrap();
        prometheus.registry()
            .register(Box::new(instance.index_counter.clone()))
            .unwrap();
        prometheus.registry()
            .register(Box::new(instance.nutrients_counter.clone()))
            .unwrap();

        return (prometheus, instance);
    }
}

struct CachesState {
    foods: Vec<Food>,
    foods_and_nutrients: HashMap<i32, Vec<JoinResult2>>,
}

impl CachesState {
    fn new() -> CachesState {
        // Create new instance
        let instance = CachesState {
            foods: CachesState::get_foods(),
            foods_and_nutrients: CachesState::get_nutrients(),
        };
        return instance;
    }

    fn get_foods() -> Vec<Food> {
        let conn = SqliteConnection::establish("usda.sqlite").unwrap();
        let all_foods = Food::all(&conn);
        return all_foods;
    }

    fn get_nutrients() -> HashMap<i32, Vec<JoinResult2>> {
        let conn = SqliteConnection::establish("usda.sqlite").unwrap();
        let all_nutrients_vec = Food::get_nutrients_all(&conn);
        let mut hashmap: HashMap<i32, Vec<JoinResult2>> = HashMap::with_capacity(all_nutrients_vec.len());

        all_nutrients_vec.iter()
            .for_each(|x| {
                let food_id = x.food_id;
                let clone = x.clone();
                let v1 = hashmap.get_mut(&food_id);
                match v1 {
                    Some(v2) => {
                        v2.push(clone);
                    },
                    None => {
                        hashmap.insert(x.food_id, vec![clone]);
                    }
                }
            });

        hashmap
    }
}


#[get("/")]
fn index(counter: State<PrometheusState>) -> String {
    counter.index_counter.inc();

    let msg = format!("Ready to serve!\n{}\n\n\
    /food\n\
    /food/<id>\n\
    /food/<id>/nutrients\n", get_version());

    msg
}

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

#[get("/ip")]
fn ip_man(remote_addr: SocketAddr) -> String {
    format!("Remote Address: {}", remote_addr.ip())
}

#[get("/food")]
fn get_all_foods(prometheus_state: State<PrometheusState>, cache_state: State<CachesState>, ) -> Json<Vec<Food>> {
    prometheus_state.allfoods_counter.inc();

    Json(cache_state.foods.clone())
}

// TODO Use Hashmap for fast lookup instead of iterating through whole array.
// TODO Can be HashMap<food_id> -> arrayindex.
#[get("/food/<food_id>")]
fn get_food_by_id(state: State<CachesState>, food_id: i32) -> Json<Option<Food>> {
    let food_opt = state.foods
        .iter()
        .find(|x| x.id == food_id);
    return match food_opt {
        Some(food) => {
            Json(Some(food.clone()))
        },
        None => {
            Json(Option::None)
        }
    }
}

#[get("/food/<food_id>/nutrients")]
fn get_nutrients(prometheus_state: State<PrometheusState>, cache_state: State<CachesState>, food_id: i32) -> Option<Json<Vec<JoinResult2>>> {
    prometheus_state.nutrients_counter.inc();

    return match cache_state.foods_and_nutrients.get(&food_id) {
        Some(x) => Some(Json(x.clone())),
        None => None
    }
}

#[get("/v2/food/<food_id>/nutrients")]
fn get_nutrients_v2(prometheus_state: State<PrometheusState>, cache_state: State<CachesState>, food_id: i32) -> Option<Json<FoodNutrients>> {
    prometheus_state.nutrients_counter.inc();

    return match cache_state.foods_and_nutrients.get(&food_id) {
        Some(nutrients) => {
            match map_to_food_nutrients(nutrients) {
                Some (nutrients_v2) => {
                    Some(Json(nutrients_v2.clone()))
                },
                None => {
                    None
                }
            }
        },
        None => {
            None
        }
    }
}

// TODO check the search string! SQL Injection!
// TODO Write a Rocket Request Guard for the search_string
#[get("/food?<search_string>")]
fn search_food(search_string: &RawStr, conn: USDADbConn) -> Json<Vec<Food>> {
    let search = format!("%{}%", search_string.as_str());
    let t1 = Food::search(&*conn, search.to_string());
    return Json(t1);
}

#[get("/foodgroup")]
fn get_all_foodgroups(conn: USDADbConn, counter: State<PrometheusState>) -> Json<Vec<FoodGroup>> {
    counter.allfoodgroups_counter.inc();

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


fn main() {
    let prometheus = PrometheusMetrics::new();
    let (prometheus, prometheus_state) = PrometheusState::new(prometheus);

    rocket::ignite()
        .mount("/", routes![index,
            get_all_foods,
            get_food_by_id,
            get_nutrients,
            get_nutrients_v2,
            get_all_foodgroups,
            get_foodgroup_by_id,
            get_joined_food_groups,
            ip_man
            ])
        .mount("/metrics", prometheus)
        .attach(USDADbConn::fairing())
        .attach(CorsFairing)
        .manage(prometheus_state)
        .launch();
}


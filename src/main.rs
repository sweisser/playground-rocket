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

#[database("usda")]
struct USDADbConn(diesel::SqliteConnection);

// TODO Split prometheus counters and cache
struct GlobalAppState {
    allfoods_counter: IntCounter,
    allfoodgroups_counter: IntCounter,
    index_counter: IntCounter,

    // caches
    foods: Vec<Food>,
    foods_and_nutrients: HashMap<i32, Vec<JoinResult2>>,
}

impl GlobalAppState {
    fn new(prometheus: PrometheusMetrics) -> (PrometheusMetrics, GlobalAppState) {
        // Create new instance
        let instance = GlobalAppState {
            index_counter: IntCounter::new("index_counter", "index_counter")
                .expect("Could not create IntCounter"),
            allfoods_counter: IntCounter::new("allfoods_counter", "allfoods_counter")
                .expect("Could not create IntCounter"),
            allfoodgroups_counter: IntCounter::new("allfoodgroups_counter", "allfoodgroups_counter")
                .expect("Could not create IntCounter"),
            foods: GlobalAppState::get_foods(),
            foods_and_nutrients: GlobalAppState::get_nutrients(),
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

        return (prometheus, instance);
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
fn index(counter: State<GlobalAppState>) -> String {
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
fn get_all_foods(state: State<GlobalAppState>) -> Json<Vec<Food>> {
    state.allfoods_counter.inc();

    Json(state.foods.clone())
}

// TODO Use Hashmap for fast lookup. Can be HashMap<food_id> -> arrayindex
#[get("/food/<food_id>")]
fn get_food_by_id(state: State<GlobalAppState>, food_id: i32) -> Json<Option<Food>> {
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
fn get_nutrients(state: State<GlobalAppState>, food_id: i32) -> Json<Vec<JoinResult2>> {
    let nutrients = state.foods_and_nutrients.get(&food_id).unwrap();
    Json(nutrients.clone())
}

// TODO check the search string! SQL Injection!
#[get("/food?<search_string>")]
fn search_food(search_string: &RawStr, conn: USDADbConn) -> Json<Vec<Food>> {
    let search = format!("%{}%", search_string.as_str());
    let t1 = Food::search(&*conn, search.to_string());
    return Json(t1);
}

#[get("/foodgroup")]
fn get_all_foodgroups(conn: USDADbConn, counter: State<GlobalAppState>) -> Json<Vec<FoodGroup>> {
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
    let (prometheus, global_state) = GlobalAppState::new(prometheus);

    rocket::ignite()
        .mount("/", routes![index,
            get_all_foods,
            get_food_by_id,
            get_nutrients,
            get_all_foodgroups,
            get_foodgroup_by_id,
            get_joined_food_groups,
            ip_man
            ])
        .mount("/metrics", prometheus)
        .attach(USDADbConn::fairing())
        .attach(CorsFairing)
        .manage(global_state)
        .launch();
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_search_string() {
        //let a = Fr
        //assert_eq!("abc", )
    }
}

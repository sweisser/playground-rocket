#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
// #![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use playground_rocket::models::{Food, FoodGroup, JoinResult};

#[database("usda")]
struct USDADbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "We want you to eat healthy, my friend!"
}

#[get("/foods")]
fn get_all_foods(conn: USDADbConn) -> Json<Vec<Food>> {
    let all_foods = Food::all(&*conn);
    return Json(all_foods);
}

#[get("/food/<food_id>")]
fn get_food_by_id(food_id: i32, conn: USDADbConn) -> Json<Food> {
    let food_result = Food::get_by_id(&*conn, food_id).unwrap();
    return Json(food_result);
}

#[get("/foodgroups")]
fn get_all_food_groups(conn: USDADbConn) -> Json<Vec<FoodGroup>> {
    let all_food_groups = FoodGroup::all(&*conn);
    return Json(all_food_groups);
}

#[get("/jointest")]
fn get_joined_food_groups(conn: USDADbConn) -> Json<Vec<JoinResult>> {
    let joined: Vec<JoinResult> = FoodGroup::all_foods_in_foodgroup(&*conn);
    return Json(joined);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_all_foods, get_food_by_id, get_all_food_groups, get_joined_food_groups])
        .attach(USDADbConn::fairing())
        .launch();
}


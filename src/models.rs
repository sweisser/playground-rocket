use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::{food, food_group, nutrition, nutrient};
use rocket_okapi::JsonSchema;
use crate::schema::food_group::dsl::*;


#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug, Clone, JsonSchema)]
#[table_name = "food_group"]
pub struct FoodGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Queryable, Identifiable, Associations, PartialEq, Debug, Clone, JsonSchema)]
#[belongs_to(FoodGroup, foreign_key="id")]
#[table_name = "food"]
pub struct Food {
    pub id: i32,
    pub food_group_id: i32,
    pub short_desc: String,
    pub long_desc: String,
}

#[derive(Serialize, Queryable, PartialEq, Debug, Clone, JsonSchema)]
pub struct FoodsInFoodGroup {
    pub food_group_id: i32,
    pub name: String,
    pub id: i32,
    pub short_desc: String,
    pub long_desc: String,
}

#[derive(Serialize, Queryable, PartialEq, Debug, Clone, JsonSchema)]
pub struct FoodAndNutrients {
    pub food_id: i32,
    pub short_desc: String,
    pub long_desc: String,
    pub nutrient_id: i32,
    pub name: String,
    pub amount: f32,
    pub units: String,
    pub num_decimal_places: String,
}

impl Food {
    pub fn all(conn: &SqliteConnection) -> Vec<Food> {
        crate::schema::food::dsl::food.order(crate::schema::food::dsl::id.asc())
            .load::<Food>(conn)
            .unwrap()
    }

    pub fn get_by_id(conn: &SqliteConnection, food_id: i32) -> Result<Food, Error> {
        crate::schema::food::dsl::food.filter(crate::schema::food::dsl::id.eq(food_id))
            .first(conn)
    }

    pub fn get_nutrients(conn: &SqliteConnection, food_id: i32) -> Vec<FoodAndNutrients> {
        let res = food::table
            .inner_join(nutrition::table.on(food::id.eq(nutrition::food_id)))
            .inner_join(nutrient::table.on(nutrition::nutrient_id.eq(nutrient::id)))
            .filter(food::id.eq(food_id))
            .select((food::id,
                     food::short_desc,
                     food::long_desc,
                     nutrient::id,
                     nutrient::name,
                     nutrition::amount,
                     nutrient::units,
                     nutrient::num_decimal_places))
            .load::<FoodAndNutrients>(conn).unwrap();
        return res;
    }

    pub fn get_nutrients_all(conn: &SqliteConnection) -> Vec<FoodAndNutrients> {
        let res = food::table
            .inner_join(nutrition::table.on(food::id.eq(nutrition::food_id)))
            .inner_join(nutrient::table.on(nutrition::nutrient_id.eq(nutrient::id)))
            .select((food::id,
                     food::short_desc,
                     food::long_desc,
                     nutrient::id,
                     nutrient::name,
                     nutrition::amount,
                     nutrient::units,
                     nutrient::num_decimal_places))
            .load::<FoodAndNutrients>(conn).unwrap();
        return res;
    }

    pub fn search(conn: &SqliteConnection, search_string: String) -> Vec<Food> {
        //use crate::schema::food::dsl::*;

        crate::schema::food::dsl::food.filter(crate::schema::food::dsl::short_desc.like(search_string))
            .load::<Food>(conn)
            .unwrap()
    }
}

impl FoodGroup {
    pub fn all(conn: &SqliteConnection) -> Vec<FoodGroup> {
        food_group.order(crate::schema::food_group::dsl::id.asc())
            .load::<FoodGroup>(conn)
            .unwrap()
    }

    pub fn get_by_id(conn: &SqliteConnection, foodgroup_id: i32) -> Result<FoodGroup, Error> {
        food_group.filter(id.eq(foodgroup_id))
            .first(conn)
    }

    pub fn all_foods_in_foodgroup(conn: &SqliteConnection) -> Vec<FoodsInFoodGroup> {
        food_group::table.inner_join(food::table)
            .select((food_group::id, food_group::name, food::id, food::short_desc, food::long_desc))
            .load::<FoodsInFoodGroup>(conn).unwrap()
    }
}

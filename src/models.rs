use diesel::prelude::*;
use diesel::result::Error;

type DieselResult<T> = Result<T, diesel::result::Error>;

use crate::schema::{food, food_group};

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug, Clone)]
#[table_name = "food_group"]
pub struct FoodGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Queryable, Identifiable, Associations, PartialEq, Debug, Clone)]
#[belongs_to(FoodGroup, foreign_key="id")]
#[table_name = "food"]
pub struct Food {
    pub id: i32,
    pub short_desc: String,
    pub long_desc: String,
}


#[derive(Serialize, Queryable, Debug, Clone)]
pub struct FoodGroupAndFoods {
    pub foodgroup_id: i32,
    pub foodgroup_name: String,
    pub food_id: i32,
    pub food_long_desc: String,
}


impl Food {
    pub fn all(conn: &SqliteConnection) -> Vec<Food> {
        use crate::schema::food::dsl::*;

        food.order(crate::schema::food::dsl::id.asc())
            .load::<Food>(conn)
            .unwrap()
    }

    pub fn get_by_id(conn: &SqliteConnection, food_id: i32) -> Result<Food, Error> {
        use crate::schema::food::dsl::*;

        food.filter(crate::schema::food::dsl::id.eq(food_id))
            .first(conn)
    }
}

impl FoodGroup {
    pub fn all(conn: &SqliteConnection) -> Vec<FoodGroup> {
        use crate::schema::food_group::dsl::*;

        food_group.order(crate::schema::food_group::dsl::id.asc())
            .load::<FoodGroup>(conn)
            .unwrap()
    }

    pub fn all_foods_in_foodgroup(conn: &SqliteConnection) -> DieselResult<Vec<(FoodGroup, Food)>> {
        use crate::schema::food_group::dsl::*;
        use crate::schema::food::dsl::*;

        crate::schema::food::table.inner_join(crate::schema::food_group::table)
            .load::<FoodGroup>(conn)
    }
}

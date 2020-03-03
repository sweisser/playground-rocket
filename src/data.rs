use std::collections::HashMap;
use crate::models::JoinResult2;

// The following are the data structures for the business logic tier.
#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct NutrientEntry {
    pub nutrient_id: i32,
    pub name: String,
    pub amount: f32,
    pub units: String,
    pub num_decimal_places: u8,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct FoodNutrients {
    pub food_id: i32,
    pub short_desc: String,
    pub long_desc: String,
    pub nutrients: HashMap<i32, NutrientEntry>,
}

impl FoodNutrients {
    pub fn new() -> FoodNutrients {
        FoodNutrients {
            food_id: 0,
            short_desc: "".to_string(),
            long_desc: "".to_string(),
            nutrients: Default::default()
        }
    }
}

pub fn map_to_food_nutrients(nutrient_array: &Vec<JoinResult2>) -> Option<FoodNutrients>{
    return match nutrient_array.first() {
        Some(first) => {
            let food_nutrients2 = nutrient_array.iter().fold(FoodNutrients {
                food_id: first.food_id,
                short_desc: first.short_desc.clone(),
                long_desc: first.long_desc.clone(),
                nutrients: HashMap::new(),
            }, |mut acc, x| {
                acc.nutrients.insert(x.nutrient_id, NutrientEntry {
                    nutrient_id: x.nutrient_id,
                    name: x.name.clone(),
                    amount: x.amount,
                    units: x.units.clone(),
                    num_decimal_places: x.num_decimal_places.parse().unwrap_or(0)
                });

                acc
            });

            Option::Some(food_nutrients2)
        },
        None => {
            Option::None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::models::JoinResult2;
    use crate::data::{FoodNutrients, NutrientEntry, map_to_food_nutrients};
    use std::collections::HashMap;

    #[test]
    fn test_nutrient_array_to_hash() {
        let array = vec![
            JoinResult2 {
                food_id: 1,
                short_desc: "food1".to_string(),
                long_desc: "food1_long".to_string(),
                nutrient_id: 100,
                name: "nutrient100".to_string(),
                amount: 125.0,
                units: "g".to_string(),
                num_decimal_places: "2".to_string()
            },
            JoinResult2 {
                food_id: 1,
                short_desc: "food1".to_string(),
                long_desc: "food1_long".to_string(),
                nutrient_id: 101,
                name: "nutrient101".to_string(),
                amount: 25.0,
                units: "g".to_string(),
                num_decimal_places: "2".to_string()
            },
            JoinResult2 {
                food_id: 1,
                short_desc: "food1".to_string(),
                long_desc: "food1_long".to_string(),
                nutrient_id: 105,
                name: "nutrient105".to_string(),
                amount: 10.0,
                units: "mg".to_string(),
                num_decimal_places: "2".to_string()
            }
        ];

        let mut expected = FoodNutrients {
            food_id: 1,
            short_desc: "food1".to_string(),
            long_desc: "food1_long".to_string(),
            nutrients: HashMap::new(),
        };

        expected.nutrients.insert(100, NutrientEntry {
            nutrient_id: 100,
            name: "nutrient100".to_string(),
            amount: 125.0,
            units: "g".to_string(),
            num_decimal_places: 2
        });

        expected.nutrients.insert(101, NutrientEntry {
            nutrient_id: 101,
            name: "nutrient101".to_string(),
            amount: 25.0,
            units: "g".to_string(),
            num_decimal_places: 2
        });
        expected.nutrients.insert(105, NutrientEntry {
            nutrient_id: 105,
            name: "nutrient105".to_string(),
            amount: 10.0,
            units: "mg".to_string(),
            num_decimal_places: 2
        });

        assert_eq!(expected, map_to_food_nutrients(array).unwrap());
    }

    #[test]
    fn test_nutrient_array_to_hash_empty_array() {
        let result = map_to_food_nutrients(vec![]);
        assert!(result.is_none())
    }
}

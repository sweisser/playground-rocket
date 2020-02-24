
table! {
    food (id) {
        id -> Integer,
        food_group_id -> Integer,
        long_desc -> Text,
        short_desc -> Text,
    }
}

table! {
    food_group (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    nutrient (id) {
        id -> Integer,
        units -> Text,
        tagname -> Text,
        name -> Text,
        num_decimal_places -> Text,
        sr_order -> Integer,
    }
}

table! {
    nutrition (food_id, nutrient_id) {
        food_id -> Integer,
        nutrient_id -> Integer,
        amount -> Float,
        num_data_points -> Integer,
        std_error -> Nullable<Float>,
        source_code -> Text,
        derivation_code -> Nullable<Text>,
        reference_food_id -> Nullable<Binary>,
        added_nutrient -> Nullable<Text>,
        num_studients -> Nullable<Integer>,
        min -> Nullable<Float>,
        max -> Nullable<Float>,
        degrees_freedom -> Nullable<Integer>,
        lower_error_bound -> Nullable<Float>,
        upper_error_bound -> Nullable<Float>,
        comments -> Nullable<Text>,
        modification_date -> Nullable<Text>,
        confidence_code -> Nullable<Text>,
    }
}

joinable!(nutrition -> food (food_id));
joinable!(food -> food_group (food_group_id));
joinable!(nutrition -> nutrient (nutrient_id));

allow_tables_to_appear_in_same_query!(
    food,
    food_group,
    nutrient,
    nutrition,
);


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

joinable!(food -> food_group (food_group_id));

allow_tables_to_appear_in_same_query!(
    food,
    food_group,
);

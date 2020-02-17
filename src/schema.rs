
table! {
    food (id) {
        id -> Integer,
        short_desc -> Text,
        long_desc -> Text,
    }
}

table! {
    food_group (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(food -> food_group (food_group_id));

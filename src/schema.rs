// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
        complete_date -> Nullable<Timestamp>,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    uploads (id) {
        id -> Text,
        filename -> Text,
        timestamp -> BigInt,
        size -> Integer,
        hash -> Text,
    }
}

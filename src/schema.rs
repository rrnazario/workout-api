// @generated automatically by Diesel CLI.

diesel::table! {
    exercise (id) {
        id -> Int4,
        name -> Varchar,
        deleted -> Bool,
        lastudpdatedate -> Nullable<Varchar>,
    }
}

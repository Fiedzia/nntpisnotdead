// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> BigInt,
        name -> Varchar,
        email -> Varchar,
        //created
    }
}

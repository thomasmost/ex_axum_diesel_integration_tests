// @generated automatically by Diesel CLI.

diesel::table! {
    org (id) {
        #[max_length = 36]
        id -> Char,
        #[max_length = 199]
        name -> Varchar,
        created -> Bigint,
    }
}

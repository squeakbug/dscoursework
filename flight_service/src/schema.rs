// @generated automatically by Diesel CLI.

diesel::table! {
    airport (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
    }
}

diesel::table! {
    flight (id) {
        id -> Int4,
        #[max_length = 20]
        flight_number -> Varchar,
        datetime -> Timestamptz,
        from_airport_id -> Nullable<Int4>,
        to_airport_id -> Nullable<Int4>,
        price -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(airport, flight,);

// @generated automatically by Diesel CLI.

diesel::table! {
    privilege (id) {
        id -> Int4,
        #[max_length = 80]
        username -> Varchar,
        #[max_length = 80]
        status -> Varchar,
        balance -> Nullable<Int4>,
    }
}

diesel::table! {
    privilege_history (id) {
        id -> Int4,
        privilege_id -> Nullable<Int4>,
        ticket_uid -> Uuid,
        datetime -> Timestamp,
        balance_diff -> Int4,
        #[max_length = 20]
        operation_type -> Varchar,
    }
}

diesel::joinable!(privilege_history -> privilege (privilege_id));

diesel::allow_tables_to_appear_in_same_query!(privilege, privilege_history,);

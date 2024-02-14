// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sheets (id) {
        id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        pw_hash -> Text,
        profile_image_url -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(notes, sheets, users,);

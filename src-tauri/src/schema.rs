// @generated automatically by Diesel CLI.

diesel::table! {
    numbers (id) {
        id -> Nullable<Integer>,
        digits -> Text,
        carrier -> Nullable<Text>,
        email -> Nullable<Text>,
        international -> Nullable<Text>,
        national -> Nullable<Text>,
        rfc3966 -> Nullable<Text>,
        e164 -> Nullable<Text>,
        is_valid -> Bool,
    }
}

table! {
    user_interactions (id) {
        id -> Int4,
        user_id -> Int4,
        content_id -> Int4,
        user_read -> Bool,
        user_like -> Bool,
    }
}

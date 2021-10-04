table! {
    contents (id) {
        id -> Int4,
        title -> Varchar,
        story -> Varchar,
        published -> Timestamptz,
        user_id -> Int4,
    }
}

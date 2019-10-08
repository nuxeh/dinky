table! {
    urls (id) {
        id -> Integer,
        url -> Text,
        created -> Text,
        accessed -> Text,
        hits -> Integer,
    }
}

table! {
    ids (id) {
        current -> Integer,
    }
}

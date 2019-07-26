use diesel::sql_types::Text;
joinable!(comments -> bookmarks (bookmark_id));
allow_tables_to_appear_in_same_query!(comments, bookmarks);

sql_function! {
    fn julianday(t: Text) -> Float;
}
sql_function! {
    fn date(t: Text) -> Text;
}

table! {
    bookmarks (id) {
        id -> Text,
        url -> Text,
        added -> Text,
    }
}

table! {
    comments (comment_id) {
        comment_id -> Text,
        bookmark_id -> Text,
        comment -> Text,
    }
}

use crate::schema::bookmarks;
use serde_derive::Serialize;

#[derive(Debug, Clone, Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookmark<'a> {
    pub id: &'a str,
    pub url: &'a str,
}

#[derive(Serialize, Queryable)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
}

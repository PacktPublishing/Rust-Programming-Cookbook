use crate::schema::{bookmarks, comments};
use serde_derive::Serialize;

#[derive(Debug, Clone, Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookmark<'a> {
    pub id: &'a str,
    pub url: &'a str,
    pub added: &'a str,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub added: String,
}

#[derive(Serialize, Queryable)]
pub struct JulianBookmark {
    pub id: String,
    pub url: String,
    pub julian: f32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Comment {
    pub bookmark_id: String,
    pub comment_id: String,
    pub comment: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub bookmark_id: &'a str,
    pub comment_id: &'a str,
    pub comment: &'a str,
}

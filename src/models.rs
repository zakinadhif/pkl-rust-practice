use diesel::Queryable;
use super::schema::notes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, AsChangeset, Deserialize, Serialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name="notes"]
pub struct NoteForm {
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub body: &'a str
}

use diesel::prelude::*;

use crate::models::{
    Note,
    NewNote,
    NoteForm
};
use crate::schema;

use schema::notes::dsl::*;

pub fn index_notes(conn: &PgConnection) -> QueryResult<Vec<Note>> {
    notes.load(conn)
}

pub fn insert_new_note(conn: &PgConnection, new_note: NewNote) -> QueryResult<()> {
    diesel::insert_into(notes)
        .values(&new_note)
        .execute(conn)?;

    Ok(())
}

pub fn remove_note(conn: &PgConnection, note: &Note) -> QueryResult<()> {
    remove_note_with_id(conn, note.id)
}

pub fn remove_note_with_id(conn: &PgConnection, note_id: i32) -> QueryResult<()> {
    diesel::delete(notes.filter(id.eq(note_id)))
        .execute(conn)?;

    Ok(())
}

pub fn update_note(conn: &PgConnection, note_id: i32, note_form: &NoteForm) -> QueryResult<()> {
    diesel::update(notes.find(note_id))
        .set(note_form)
        .execute(conn)?;

    Ok(())
}

pub fn read_note(conn: &PgConnection, note_id: i32) -> QueryResult<Note> {
    notes.find(note_id)
        .get_result(conn)
}

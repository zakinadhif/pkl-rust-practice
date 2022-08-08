use diesel::prelude::*;

use crate::models::*;
use crate::schema;

pub fn index_notes(conn: &PgConnection) -> QueryResult<Vec<Note>> {
    use schema::notes::dsl::*;

    notes.load(conn)
}

pub fn insert_new_note(conn: &PgConnection, new_note: NewNote) -> QueryResult<()> {
    use schema::notes::dsl::*;

    diesel::insert_into(notes)
        .values(&new_note)
        .execute(conn)?;

    Ok(())
}

pub fn remove_note(conn: &PgConnection, note: Note) -> QueryResult<()> {
    remove_note_with_id(conn, note.id)
}

pub fn remove_note_with_id(conn: &PgConnection, note_id: i32) -> QueryResult<()> {
    use schema::notes::dsl::*;

    diesel::delete(notes.filter(id.eq(note_id)))
        .execute(conn)?;

    Ok(())
}

pub fn update_note(conn: &PgConnection, note_id: i32, note_form: &NoteForm) -> QueryResult<()> {
    use schema::notes::dsl::*;

    diesel::update(notes.find(note_id))
        .set(note_form)
        .execute(conn)?;

    Ok(())
}

pub fn read_note(conn: &PgConnection, note_id: i32) -> QueryResult<Note> {
    use schema::notes::dsl::*;

    notes.find(note_id)
        .get_result(conn)
}

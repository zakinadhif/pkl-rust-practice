use actix_web::{web, Responder, HttpResponse, error, Result};

use crate::AppState;
use goodbye_world::{
    actions::note as actions,
    models::{NoteForm, NewNote}
};

pub async fn index(app_state: web::Data<AppState>) -> impl Responder {
    match actions::index_notes(&app_state.db) {
        Ok(notes) => {
            Ok(web::Json(notes))
        },
        Err(e) => {
            eprintln!("{}", e);
            Err(error::ErrorInternalServerError(e))
        }
    }
}

pub async fn read(app_state: web::Data<AppState>, note_id: web::Path<i32>) -> Result<impl Responder> {
    let note_id = note_id.into_inner();

    match actions::read_note(&app_state.db, note_id) {
        Ok(note) => {
            Ok(web::Json(note))
        },
        Err(e) => {
            eprintln!("{}", e);
            Err(error::ErrorInternalServerError(e))
        }
    }
}

pub async fn insert(app_state: web::Data<AppState>, note: web::Json<NoteForm>) -> impl Responder {
    let new_note = NewNote {
        title: note.title.as_str(),
        body: note.title.as_str()
    };

    if let Err(e) = actions::insert_new_note(&app_state.db, new_note) {
        eprintln!("{}", e);
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn update(app_state: web::Data<AppState>, note_form: web::Json<NoteForm>, note_id: web::Path<i32>) -> impl Responder {
    let note_id = note_id.into_inner();

    if let Err(e) = actions::update_note(&app_state.db, note_id, &note_form) {
        eprintln!("{}", e);
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn remove(app_state: web::Data<AppState>, note_id: web::Path<i32>) -> impl Responder {
    let note_id = note_id.into_inner();

    if let Err(e) = actions::remove_note_with_id(&app_state.db, note_id) {
        eprintln!("{}", e);
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}


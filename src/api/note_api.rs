//Importation des modules
use crate::{model::note_model::Note, repository::mongo_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, get, put, delete,
};
use mongodb::bson::oid::ObjectId;

//Route pour créer une note
#[post("/note")]
pub async fn create_note(db: Data<MongoRepo>, new_note: Json<Note>) -> HttpResponse {
    //Set du data à partir du body
    let data = Note {
        id: None,
        student_id: new_note.student_id.to_owned(),
        evaluation_id: new_note.evaluation_id.to_owned(),
        note: new_note.note.to_owned()
    };
    //opération db
    let note_detail = db.create_note(data).await;
    match note_detail {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour aller chercher une note par étudiant et évaluation
#[get("/note/eval/{student_id}/{evaluation_id}")]
pub async fn get_note_for_student_eval(db: Data<MongoRepo>, path: Path<(String,String)>) -> HttpResponse {
    //Set des id d'étudiant et d'évaluation à partir des path params
    let (student_id, evaluation_id) = path.into_inner();

    if student_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid student id");
    }
    if evaluation_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid evaluation id");
    }

    //Opération db
    let note_detail = db.get_note_for_student_eval(&student_id, &evaluation_id).await;
    match note_detail {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour aller chercher une note par id
#[get("/note/{id}")]
pub async fn get_note(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set de l'id à partir des path params
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    //opération db
    let note_detail = db.get_note(&id).await;
    match note_detail {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour modifier une note par id
#[put("/note/{id}")]
pub async fn update_note(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_note: Json<Note>,
) -> HttpResponse {
    //Set de l'id à partir du path param
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Set du data à partir du body
    let data = Note {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        student_id: new_note.student_id.to_owned(),
        evaluation_id: new_note.evaluation_id.to_owned(),
        note: new_note.note.to_owned()
    };
    //Opération db
    let update_result = db.update_note(&id, data).await;
    match update_result {
        Ok(update) => {
            //Si ça a été update, va chercher le document updaté et le retourne
            if update.matched_count == 1 {
                let updated_student_info = db.get_note(&id).await;
                return match updated_student_info {
                    Ok(note) => HttpResponse::Ok().json(note),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No note found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour supprimer une note par id
#[delete("/note/{id}")]
pub async fn delete_note(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set du id à partir des path params
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Opération db
    let result = db.delete_note(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Note successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Note with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour aller chercher toutes les notes d'une évaluation
#[get("/notes/{evaluation_id}")]
pub async fn get_all_notes(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set de l'id d'évaluation à partir des path params
    let evaluation_id = path.into_inner();
    //Opération db
    let notes = db.get_all_notes(&evaluation_id).await;
    match notes {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
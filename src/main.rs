mod api;
mod model;
mod repository;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
use api::student_api::{
    create_student, 
    get_student, 
    update_student, 
    delete_student, 
    get_all_students
};
use api::evaluation_api::{
    create_evaluation,
    get_evaluation,
    update_evaluation,
    delete_evaluation,
    get_all_evaluations
};
use api::note_api::{
    create_note,
    get_note,
    get_note_for_student_eval,
    update_note,
    delete_note,
    get_all_notes
};
use repository::mongo_repo::MongoRepo;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and patate")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(index)
            .service(create_student)
            .service(get_student) 
            .service(update_student)
            .service(delete_student)
            .service(get_all_students)
            .service(create_evaluation)
            .service(get_evaluation)
            .service(update_evaluation)
            .service(delete_evaluation)
            .service(get_all_evaluations)
            .service(create_note)
            .service(get_note)
            .service(get_note_for_student_eval)
            .service(update_note)
            .service(delete_note)
            .service(get_all_notes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
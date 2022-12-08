//Inclusion des namespace
mod api;
mod model;
mod repository;

//Inclusion des modules utilis«s
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
//Module de routes des étudiants
use api::student_api::{
    create_student, 
    get_student, 
    update_student, 
    delete_student, 
    get_all_students
};
//Module de route des évaluations
use api::evaluation_api::{
    create_evaluation,
    get_evaluation,
    update_evaluation,
    delete_evaluation,
    get_all_evaluations
};
//Module de routes des notes
use api::note_api::{
    create_note,
    get_note,
    get_note_for_student_eval,
    update_note,
    delete_note,
    get_all_notes
};
//Repo mongodb
use repository::mongo_repo::MongoRepo;

//Route principale
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Api en Rust pour le cours de Technologies émergentes - par Jonathan  Chiasson")
}

//Programme principal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //init du repo pour la bd
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    //Init du server http
    HttpServer::new(move || {
        App::new()
        //Ajout des données de la bd pour être partages partout
            .app_data(db_data.clone())
        //Ajout de toutes les routes
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
    //Attache au port et démarrage
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
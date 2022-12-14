//Importation des modules
use crate::{model::student_model::Student, repository::mongo_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, get, put, delete,
};
use mongodb::bson::oid::ObjectId;

//Route pour ajouter un étudiant
#[post("/student")]
pub async fn create_student(db: Data<MongoRepo>, new_student: Json<Student>) -> HttpResponse {
    //Set du data à partir du body
    let data = Student {
        id: None,
        name: new_student.name.to_owned(),
        admission_number: new_student.admission_number.to_owned(),
    };
    //opération db
    let student_detail = db.create_student(data).await;
    match student_detail {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour aller chercher un étudiant par id
#[get("/student/{id}")]
pub async fn get_student(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set de l'id à partir des path params
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    //opération db
    let student_detail = db.get_student(&id).await;
    match student_detail {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour modifier un étudiant par id
#[put("/student/{id}")]
pub async fn update_student(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_student: Json<Student>,
) -> HttpResponse {
    //Set de l'id à partir des path params
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Set du data à partir du body
    let data = Student {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_student.name.to_owned(),
        admission_number: new_student.admission_number.to_owned(),
    };
    //Opération db
    let update_result = db.update_student(&id, data).await;
    match update_result {
        Ok(update) => {
            //Si l'étudiant a été updaté, va chercher le document updaté et le retourne
            if update.matched_count == 1 {
                let updated_student_info = db.get_student(&id).await;
                return match updated_student_info {
                    Ok(student) => HttpResponse::Ok().json(student),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No student found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour supprimer un étudiant par id
#[delete("/student/{id}")]
pub async fn delete_student(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set de l'id à partir des path params
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Opération db
    let result = db.delete_student(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Student successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Student with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour avoir tous les étudiants
#[get("/student")]
pub async fn get_all_students(db: Data<MongoRepo>) -> HttpResponse {
    //Opération db
    let students = db.get_all_students().await;
    match students {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
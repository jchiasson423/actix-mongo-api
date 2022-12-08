//Importation des modules
use crate::{model::evaluation_model::Evaluation, repository::mongo_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, get, put, delete,
};
use mongodb::bson::oid::ObjectId;

//Route pour créer une évaluation
#[post("/evaluation")]
pub async fn create_evaluation(db: Data<MongoRepo>, new_evaluation: Json<Evaluation>) -> HttpResponse {
    //Set du data avec le body
    let data = Evaluation {
        id: None,
        name: new_evaluation.name.to_owned(),
        class: new_evaluation.class.to_owned(),
    };
    //Opération dans la db
    let evaluation_detail = db.create_evaluation(data).await;
    match evaluation_detail {
        Ok(evaluation) => HttpResponse::Ok().json(evaluation),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour aller chercher une évaluation par id
#[get("/evaluation/{id}")]
pub async fn get_evaluation(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set du id à partir du path param
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    //Opération dans la db
    let evaluation_detail = db.get_evaluation(&id).await;
    match evaluation_detail {
        Ok(evaluation) => HttpResponse::Ok().json(evaluation),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour modifier une évaluation par id
#[put("/evaluation/{id}")]
pub async fn update_evaluation(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_evaluation: Json<Evaluation>,
) -> HttpResponse {
    //Set du id à partir du path param
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Set du data à partir du body
    let data = Evaluation {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_evaluation.name.to_owned(),
        class: new_evaluation.class.to_owned(),
    };
    //Opération db
    let update_result = db.update_evaluation(&id, data).await;
    match update_result {
        Ok(update) => {
            //Si ça a été update, va chercher les données du document et les envoie
            if update.matched_count == 1 {
                let updated_evaluation_info = db.get_evaluation(&id).await;
                return match updated_evaluation_info {
                    Ok(evaluation) => HttpResponse::Ok().json(evaluation),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No evaluation found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour supprimer une évaluation par id
#[delete("/evaluation/{id}")]
pub async fn delete_evaluation(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //Set du id à partir du path param
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    //Opération db
    let result = db.delete_evaluation(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Evaluation successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Evaluation with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Route pour avoir toutes les évaluations
#[get("/evaluation")]
pub async fn get_all_evaluations(db: Data<MongoRepo>) -> HttpResponse {
    //Opération db
    let evaluations = db.get_all_evaluations().await;
    match evaluations {
        Ok(evaluations) => HttpResponse::Ok().json(evaluations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
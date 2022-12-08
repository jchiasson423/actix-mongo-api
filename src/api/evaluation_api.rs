use crate::{model::evaluation_model::Evaluation, repository::mongo_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, get, put, delete,
};
use mongodb::bson::oid::ObjectId;

#[post("/evaluation")]
pub async fn create_evaluation(db: Data<MongoRepo>, new_evaluation: Json<Evaluation>) -> HttpResponse {
    let data = Evaluation {
        id: None,
        name: new_evaluation.name.to_owned(),
        class: new_evaluation.class.to_owned(),
    };
    let evaluation_detail = db.create_evaluation(data).await;
    match evaluation_detail {
        Ok(evaluation) => HttpResponse::Ok().json(evaluation),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/evaluation/{id}")]
pub async fn get_evaluation(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let evaluation_detail = db.get_evaluation(&id).await;
    match evaluation_detail {
        Ok(evaluation) => HttpResponse::Ok().json(evaluation),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/evaluation/{id}")]
pub async fn update_evaluation(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_evaluation: Json<Evaluation>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Evaluation {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_evaluation.name.to_owned(),
        class: new_evaluation.class.to_owned(),
    };
    let update_result = db.update_evaluation(&id, data).await;
    match update_result {
        Ok(update) => {
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

#[delete("/evaluation/{id}")]
pub async fn delete_evaluation(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
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

#[get("/evaluation")]
pub async fn get_all_evaluations(db: Data<MongoRepo>) -> HttpResponse {
    let evaluations = db.get_all_evaluations().await;
    match evaluations {
        Ok(evaluations) => HttpResponse::Ok().json(evaluations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
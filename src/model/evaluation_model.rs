//Importation des modules
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

//Structure pour évaluation
#[derive(Debug, Serialize, Deserialize)]
pub struct Evaluation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub class: String,
}
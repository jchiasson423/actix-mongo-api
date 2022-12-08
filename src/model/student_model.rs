//Importation des modules
use mongodb::bson::{oid::ObjectId,};
use serde::{Serialize, Deserialize};

//Structure pour étudiant
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub admission_number: i32,
}
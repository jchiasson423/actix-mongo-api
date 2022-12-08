//Importation des modules
use mongodb::bson::{oid::ObjectId, Decimal128};
use serde::{Serialize, Deserialize};

//Structure pour note
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub student_id: String,
    pub evaluation_id: String,
    pub note: Decimal128,
}
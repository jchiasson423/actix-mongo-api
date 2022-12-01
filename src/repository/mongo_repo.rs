use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use crate::{model::{student_model::Student, evaluation_model::Evaluation, note_model::Note}};

pub struct MongoRepo {
    col_student: Collection<Student>,
    col_evaluation: Collection<Evaluation>,
    col_note: Collection<Note>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = "mongodb+srv://admin:mLov6QfW1uxGkg6R@appdev.gce7a7b.mongodb.net/?retryWrites=true&w=majority";
        let database_string = "techno_emergentes";
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&database_string);
        let col_student: Collection<Student> = db.collection("students");
        let col_evaluation: Collection<Evaluation> = db.collection("evaluations");
        let col_note: Collection<Note> = db.collection("notes");
        MongoRepo { col_student, col_evaluation, col_note }
    }

    pub async fn create_student(&self, new_student:Student) -> Result<InsertOneResult, Error> {
        let new_doc = Student {
            id: None,
            name: new_student.name,
            admission_number: new_student.admission_number,
        };
        let student = self
            .col_student
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating student");
        Ok(student)
    }

    pub async fn get_student(&self, id: &String) -> Result<Student, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .col_student
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting student's detail");
        Ok(student_detail.unwrap())
    }

    pub async fn update_student(&self, id: &String, new_student:Student) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_student.id,
                    "name": new_student.name,
                    "admission_number": new_student.admission_number,
                },
        };
        let updated_doc = self
            .col_student
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating student");
        Ok(updated_doc)
    }

    pub async fn delete_student(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .col_student
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(student_detail)
    }

    pub async fn get_all_students(&self) -> Result<Vec<Student>, Error> {
        let mut cursors = self
            .col_student
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of students");
        let mut students: Vec<Student> = Vec::new();
        while let Some(student) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            students.push(student)
        }
        Ok(students)
    }
    
}
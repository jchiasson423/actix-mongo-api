//Importation des modules
extern crate dotenv;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use crate::{model::{student_model::Student, evaluation_model::Evaluation, note_model::Note}};

//Struct des collections
pub struct MongoRepo {
    col_student: Collection<Student>,
    col_evaluation: Collection<Evaluation>,
    col_note: Collection<Note>,
}

//Repo qui va communiquer avec la bd mongo
impl MongoRepo {

    //Init de la bd
    pub async fn init() -> Self {
        dotenv().ok();
        //Urlrl de connexion
        let uri = "mongodb+srv://admin:mLov6QfW1uxGkg6R@appdev.gce7a7b.mongodb.net/?retryWrites=true&w=majority";
        //Base de données à laquelle se connecter
        let database_string = "techno_emergentes";
        //Set de la connexion
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&database_string);
        //Set des collections
        let col_student: Collection<Student> = db.collection("students");
        let col_evaluation: Collection<Evaluation> = db.collection("evaluations");
        let col_note: Collection<Note> = db.collection("notes");
        MongoRepo { col_student, col_evaluation, col_note }
    }

    /**
     * Etudiants
     */

    //Crée un étudiant
    //new_student: Student nouvel étudient
    pub async fn create_student(&self, new_student:Student) -> Result<InsertOneResult, Error> {
        //Nouveau document
        let new_doc = Student {
            id: None,
            name: new_student.name,
            admission_number: new_student.admission_number,
        };
        //Opération mongo
        let student = self
            .col_student
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating student");
        Ok(student)
    }

    //Obtient un étudiant par id
    //id: String id de l'étudiant
    pub async fn get_student(&self, id: &String) -> Result<Student, Error> {
        //Set de l'object id
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Set de l'object id
        let filter = doc! {"_id": obj_id};
        //Opération mongo
        let student_detail = self
            .col_student
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting student's detail");
        Ok(student_detail.unwrap())
    }

    //Met à jour un étudiant par id
    //id:String id de l'étudiant
    //new_student: Student nouvel étudiant
    pub async fn update_student(&self, id: &String, new_student:Student) -> Result<UpdateResult, Error> {
        //Set de l'object id
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filte de recherche
        let filter = doc! {"_id": obj_id};
        //Nouveau document
        let new_doc = doc! {
            "$set":
                {
                    "id": new_student.id,
                    "name": new_student.name,
                    "admission_number": new_student.admission_number,
                },
        };
        //Opération mongo
        let updated_doc = self
            .col_student
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating student");
        Ok(updated_doc)
    }

    //Supprime un étudiant par id
    //id:String id de l'étudiant
    pub async fn delete_student(&self, id: &String) -> Result<DeleteResult, Error> {
        //Set de l'object id
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filte de recherche
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .col_student
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting student");
        //Opération mongo
        Ok(student_detail)
    }

    //Obtiens tous les étudiants
    pub async fn get_all_students(&self) -> Result<Vec<Student>, Error> {
        //opération mongo
        let mut cursors = self
            .col_student
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of students");
        let mut students: Vec<Student> = Vec::new();
        //Transformation en tableau
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

    
    /**
     * Évaluations
     */

    //Crée une évaluation
    //new_evalusation:Evaluation nouvelle évaluation
    pub async fn create_evaluation(&self, new_evaluation:Evaluation) -> Result<InsertOneResult, Error> {
        //nouveau document
        let new_doc = Evaluation {
            id: None,
            name: new_evaluation.name,
            class: new_evaluation.class,
        };
        //opération mongo
        let evaluation = self
            .col_evaluation
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating evalutation");
        Ok(evaluation)
    }

    //obtient une évaluation par id
    //id:String id de l'évaluation
    pub async fn get_evaluation(&self, id: &String) -> Result<Evaluation, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filtre de recherche
        let filter = doc! {"_id": obj_id};
        //opération mongo
        let evaluation_detail = self
            .col_evaluation
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting evalutation's detail");
        Ok(evaluation_detail.unwrap())
    }

    //Modifie une évaluation par id
    //id: String id de l'évaluation
    //new_evaluation:Evaluation nouvelle évaluation
    pub async fn update_evaluation(&self, id: &String, new_evaluation:Evaluation) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filtre de recherche
        let filter = doc! {"_id": obj_id};
        //Nouveau document
        let new_doc = doc! {
            "$set":
                {
                    "id": new_evaluation.id,
                    "name": new_evaluation.name,
                    "class": new_evaluation.class
                },
        };
        //Opération mongo
        let updated_doc = self
            .col_evaluation
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating evalutation");
        Ok(updated_doc)
    }

    //Supprime une évaluation par id
    //id: String id de l'évaluation
    pub async fn delete_evaluation(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filtre de recherche
        let filter = doc! {"_id": obj_id};
        //opération mongo
        let evaluation_detail = self
            .col_evaluation
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting evalutation");
        Ok(evaluation_detail)
    }

    //Obtiens toutes les évaluations
    pub async fn get_all_evaluations(&self) -> Result<Vec<Evaluation>, Error> {
        //Opération mongo
        let mut cursors = self
            .col_evaluation
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of evalutations");
        let mut evalutations: Vec<Evaluation> = Vec::new();
        //Transformation en tableau
        while let Some(evalutation) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            evalutations.push(evalutation)
        }
        Ok(evalutations)
    }

    /**
     * Notes
     */

    //Obtient la note d'un étudiant pour une évaluation
    //student_id:String is de l'étudiant
    //evaluation_id:String id de l'évaluation
    pub async fn get_note_for_student_eval(&self, student_id: &String, evaluation_id: &String) -> Result<Note, Error> {
        //Filtre de recherche
        let filter = doc! {"student_id": student_id, "evaluation_id": evaluation_id};
        //opération mongo
        let note_detail = self
            .col_note
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting note's detail");
        Ok(note_detail.unwrap())
    }

    //Obtient une note par id
    //id:String id de la note
    pub async fn get_note(&self, id: &String) -> Result<Note, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filtre de recherche
        let filter = doc! {"_id": obj_id};
        //opération mongo
        let note_detail = self
            .col_note
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting note's detail");
        Ok(note_detail.unwrap())
    }

    //Crée une note
    //new_note:Note nouvelle note
    pub async fn create_note(&self, new_note:Note) -> Result<InsertOneResult, Error> {
        //Nouveau document
        let new_doc = Note {
            id: None,
            student_id: new_note.student_id,
            evaluation_id: new_note.evaluation_id,
            note: new_note.note,
        };
        //opération mongo
        let note = self
            .col_note
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating note");
        Ok(note)
    }

    //Modifie une note par id
    //id:String id de la note
    //new_note:String nouvelle note
    pub async fn update_note(&self, id: &String, new_note:Note) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //Filtr de recherche
        let filter = doc! {"_id": obj_id};
        //Nouveau document
        let new_doc = doc! {
            "$set":
                {
                    "id": new_note.id,
                    "note": new_note.note
                },
        };
        //Opération mongo
        let updated_doc = self
            .col_note
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating note");
        Ok(updated_doc)
    }

    //Supprime une note par id
    //id:String id de la note
    pub async fn delete_note(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        //filtre de recherche
        let filter = doc! {"_id": obj_id};
        //Opération mongo
        let evaluation_detail = self
            .col_note
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting note");
        Ok(evaluation_detail)
    }

    //Obtiens toutes les notes d'une évaluation
    //evaluation_id:String id de l'évaluation
    pub async fn get_all_notes(&self, evaluation_id: &String) -> Result<Vec<Note>, Error> {
        //Filtre de recherche
        let filter = doc! {"evaluation_id": evaluation_id};
        //opération mongo
        let mut cursors = self
            .col_note
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of notes");
        //Changement en tableau
        let mut notes: Vec<Note> = Vec::new();
        while let Some(note) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            notes.push(note)
        }
        Ok(notes)
    }

}
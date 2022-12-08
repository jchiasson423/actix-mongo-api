use actix_web::{get, web, test, App, HttpResponse, Error, Responder};
use student_model::Student;
use api::student_api::{
    create_student, 
    get_student, 
    update_student, 
    delete_student, 
    get_all_students
};
use repository::mongo_repo::MongoRepo;

//Test d'insertion d'un étudiant - infos correctes
#[actix_rt::test]
async fn test_student_insert_good_infos(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_student);
    );
    //Set du data
    let data = Student{
        id: None,
        name: "Unit test",
        admission_number: 1234567
    }
    //Set de la requête 
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(data)
        .to_request();
    
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test d'insertion de l'étudiant - nom incorrect
#[actix_rt::test]
async fn test_student_insert_bad_name(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_student);
    );
    //Set du data
    let data = Student{
        id: None,
        name: None,
        admission_number: 1234567
    }
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test insertion d'un étudiant - numério d'admission incorrect
#[actix_rt::test]
async fn test_student_insert_bad_admission(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_student);
    );
    //Set du data
    let data = Student{
        id: None,
        name: "Unit test",
        admission_number: None
    }
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/student")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification étudiant - infos correctes
#[actix_rt::test]
async fn test_student_update_good_infos(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_student);
    );
    //Set du data
    let data = Student{
        id: None
        name: "Unit test",
        admission_number: 1234567
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/student/638908298e59033508e76fdd")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test modification étudiant - mauvais nom
#[actix_rt::test]
async fn test_student_update_bad_name(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_student);
    );
    //Set data
    let data = Student{
        id: None
        name: None,
        admission_number: 1234567
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/student/638908298e59033508e76fdd")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification étudiant - mauvais numéro d'admission
#[actix_rt::test]
async fn test_student_update_bad_admission(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_student);
    );
    //Set du data
    let data = Student{
        id: None
        name: "Unit test",
        admission_number: None
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/student/638908298e59033508e76fdd")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification étudiant - mauvais id
#[actix_rt::test]
async fn test_student_update_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set du data
    let data = Student{
        id: None
        name: "Unit test",
        admission_number: 1234567
    }
    //Set de la requête
    let req = test::TestRequest::put()
        .uri("/student/abcdef")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test suppression étudiant - mauvais id
//Je ne teste pas le bon id pour garder mes données de test intactes
#[actix_rt::test]
async fn test_student_delete_bad_id(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set requete
    let req = test::TestRequest::delete()
        .uri("/student/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir étudiant - mauvais id
#[actix_rt::test]
async fn test_student_get_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_student);
    );
    //Set de la requete
    let req = test::TestRequest::get()
        .uri("/student/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir étudiant - bonnes infos
#[actix_rt::test]
async fn test_student_get_good_infos(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_student);
    );
    //Set requete
    let req = test::TestRequest::get()
        //Id déjà dans la bd pour les tests
        .uri("/student/638908298e59033508e76fdd")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test obtenir tous les étudiants
#[actix_rt::test]
async fn test_student_get_all_students(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_all_students);
    );
    //Set requete
    let req = test::TestRequest::get()
        .uri("/student")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}


use actix_web::{get, web, test, App, HttpResponse, Error, Responder};
use note_model::Note;
use api::note_api::{
    create_note,
    get_note,
    get_note_for_student_eval,
    update_note,
    delete_note,
    get_all_notes
};
use repository::mongo_repo::MongoRepo;

//Test d'insertion d'une note - infos correctes
#[actix_rt::test]
async fn test_note_insert_good_infos(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: "6389119749e479624966bf6b",
        note: 83
    }
    //Set de la requête 
    let req = test::TestRequest::post()
        .uri("/note")
        .set_json(data)
        .to_request();
    
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test d'insertion de la note - nom incorrect
#[actix_rt::test]
async fn test_note_insert_bad_student_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: None,
        evaluation_id: "6389119749e479624966bf6b",
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/note")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test insertion d'une note - evaluation_id incorrect
#[actix_rt::test]
async fn test_note_insert_bad_evaluation_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: None,
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/note")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test insertion d'une note - note incorrecte
#[actix_rt::test]
async fn test_note_insert_bad_note(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: "6389119749e479624966bf6b",
        note: None
    }
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/note")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification note - infos correctes
#[actix_rt::test]
async fn test_note_update_good_infos(){note
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: "6389119749e479624966bf6b",
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("note/639252cd12780e55c1d598a6")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test modification note - mauvais student_id
#[actix_rt::test]
async fn test_note_update_bad_student_id(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_note);
    );
    //Set data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: None,
        evaluation_id: "6389119749e479624966bf6b",
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/note/639252cd12780e55c1d598a6")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification note - mauvais evaluation_id
#[actix_rt::test]
async fn test_note_update_bad_evaluation_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: None,
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/note/639252cd12780e55c1d598a6")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification note - mauvaise note
#[actix_rt::test]
async fn test_note_update_bad_note(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_note);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: "6389119749e479624966bf6b",
        note: None
    }
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/note/639252cd12780e55c1d598a6")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification note - mauvais id
#[actix_rt::test]
async fn test_note_update_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set du data
    //Id présents dans la bd pour les tests
    let data = Note{
        id: None,
        student_id: "638908298e59033508e76fdd",
        evaluation_id: "6389119749e479624966bf6b",
        note: 83
    }
    //Set de la requête
    let req = test::TestRequest::put()
        .uri("note/abcdef")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test suppression note - mauvais id
//Je ne teste pas le bon id pour garder mes données de test intactes
#[actix_rt::test]
async fn test_note_delete_bad_id(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set requete
    let req = test::TestRequest::delete()
        .uri("/note/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir note - mauvais id
#[actix_rt::test]
async fn test_note_get_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_note);
    );
    //Set de la requete
    let req = test::TestRequest::get()
        .uri("/note/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir note - bonnes infos
#[actix_rt::test]
async fn test_note_get_good_infos(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_note);
    );
    //Set requete
    let req = test::TestRequest::get()
        //Id déjà présent dans la bd pour les tests
        .uri("/note/639252cd12780e55c1d598a6")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test obtenir toutes les notes
#[actix_rt::test]
async fn test_note_get_all_notes(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_all_notes);
    );
    //Set requete
    let req = test::TestRequest::get()
        //Id déjà présent dans la bd pour les tests
        .uri("/note/eval/6389119749e479624966bf6b")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}
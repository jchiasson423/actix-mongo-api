use actix_web::{get, web, test, App, HttpResponse, Error, Responder};
use rvaluation_model::Evaluation;
use api::evaluation_api::{
    create_evaluation,
    get_evaluation,
    update_evaluation,
    delete_evaluation,
    get_all_evaluations
};
use repository::mongo_repo::MongoRepo;

//Test d'insertion d'une évaluation - infos correctes
#[actix_rt::test]
async fn test_evaluation_insert_good_infos(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_evaluation);
    );
    //Set du data
    let data = Student{
        id: None,
        name: "Unit test",
        admission_number: 1234567
    }
    //Set de la requête 
    let req = test::TestRequest::post()
        .uri("/evaluation")
        .set_json(data)
        .to_request();
    
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test d'insertion de l'évaluation - nom incorrect
#[actix_rt::test]
async fn test_evaluation_insert_bad_name(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_evaluation);
    );
    //Set du data
    let data = Evaluation{
        id: None
        name: None,
        class: "Unit test class",
  	}
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/evaluation")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test insertion d'une évaluation - classe incorrecte
#[actix_rt::test]
async fn test_evaluation_insert_bad_class(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(create_evaluation);
    );
    //Set du data
    let data = Evaluation{
        id: None
        name: "Unit test",
        class: None,
  	}
    //Set de la requête
    let req = test::TestRequest::post()
        .uri("/evaluation")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification évaluation - infos correctes
#[actix_rt::test]
async fn test_evaluation_update_good_infos(){evaluation
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_evaluation);
    );
    //Set du data
    let data = Evaluation{
        id: None
        name: "Unit test",
        class: "Unit test class",
  	}
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("evaluation/6389119749e479624966bf6b")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test modification évaluation - mauvais nom
#[actix_rt::test]
async fn test_evaluation_update_bad_name(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_evaluation);
    );
    //Set data
    let data = Evaluation{
        id: None
        name: None,
        class: "Unit test class",
  	}
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/evaluation/6389119749e479624966bf6b")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification évaluation - classe incorrecte
#[actix_rt::test]
async fn test_evaluation_update_bad_class(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(update_evaluation);
    );
    //Set du data
    let data = Evaluation{
        id: None
        name: "Unit test",
        class: None,
  	}
    //Set de la requête
    let req = test::TestRequest::put()
        //id déjà présent dans la bd pour les tests
        .uri("/evaluation/6389119749e479624966bf6b")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test modification évaluation - mauvais id
#[actix_rt::test]
async fn test_evaluation_update_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set du data
    let data = Evaluation{
        id: None
        name: "Unit test",
        class: "Unit test class",
  	}
    //Set de la requête
    let req = test::TestRequest::put()
        .uri("evaluation/abcdef")
        .set_json(data)
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test suppression évaluation - mauvais id
//Je ne teste pas le bon id pour garder mes données de test intactes
#[actix_rt::test]
async fn test_evaluation_delete_bad_id(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(delete_student);
    );
    //Set requete
    let req = test::TestRequest::delete()
        .uri("/evaluation/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir évaluation - mauvais id
#[actix_rt::test]
async fn test_evaluation_get_bad_id(){
    //Set du serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_evaluation);
    );
    //Set de la requete
    let req = test::TestRequest::get()
        .uri("/evaluation/abcdef")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(!res.status().is_success());
}

//Test obtenir évaluation - bonnes infos
#[actix_rt::test]
async fn test_evaluation_get_good_infos(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_evaluation);
    );
    //Set requete
    let req = test::TestRequest::get()
        //Id déjà dans la bd pour les tests
        .uri("/evaluation/6389119749e479624966bf6b")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}

//Test obtenir toutes les évaluations
#[actix_rt::test]
async fn test_evaluation_get_all_evaluations(){
    //Set serveur
    let srv = actix_test::start(||
        App::new().app_data(db_data.clone()).service(get_all_students);
    );
    //Set requete
    let req = test::TestRequest::get()
        .uri("/evaluation")
        .to_request();
    //Test et vérification
    let res = test::call_servile(&app, req).await;
    assert!(res.status().is_success());
}
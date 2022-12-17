use log::info;
use actix_web::{HttpRequest, HttpResponse, Responder, web, HttpResponseBuilder};
use fireauth::FireAuth;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
}

pub async fn firebase_signup(payload: web::Json<User>) -> impl Responder{
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    let email = &payload.email;
    let password = &payload.password;
    let responce = match auth.sign_up_email(&email, &password, true).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    info!("id_token: {:?}", responce.id_token);
    info!("email: {:?}", responce.email);
    HttpResponse::Ok().body("OK")
}

pub async fn firebase_signin(payload: web::Json<User>) -> impl Responder{
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    let email = &payload.email;
    let password = &payload.password;
    let responce = match auth.sign_in_email(&email, &password, true).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    info!("id_token: {:?}", responce.id_token);
    info!("email: {:?}", responce.email);
    HttpResponse::Ok().body("OK")
}

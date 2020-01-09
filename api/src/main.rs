use actix_web::{web, App, HttpServer, Responder};

mod person_repository;
mod person_controller;

mod cult_repository;
mod cult_controller;

mod types;

fn greet() -> impl Responder {
    "Yo there"
}

fn main() {
    println!("Server starting...");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello")
                .route(web::get().to(greet)))
            .service(web::resource("/persons")
                .route(web::get().to(person_controller::get_person_list)))
            .service(web::resource("/persons/{person_id}")
                .route(web::get().to(person_controller::get_person)))
                // .route(web::post().to(create_person)))
                // .route(web::delete().to(delete_person))
                // .route(web::put().to(update_person)))
            .service(web::resource("/cults/{cult_id}")
                .route(web::get().to(cult_controller::get_cult)))
            .service(web::resource("/cults")
                .route(web::get().to(cult_controller::get_cult_list)))
    })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .unwrap();
}

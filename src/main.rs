#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
use pretty_env_logger;

use std::rc::Rc;
use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},

};
use sqlx::{postgres::PgPoolOptions, Postgres,PgPool};

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();
    println!("Server starting....{}", "Starting");
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect_lazy(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
    //     .expect("Failed to create Postgres connection pool!");

    // let answers_dao = AnswersDaoImpl::new(pool);
    println!("Started");
    rocket::build()
    .mount(
        "/",
        routes![
            hello_world,
            
        
        ],
    )
        .attach(CORS)
        // .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}
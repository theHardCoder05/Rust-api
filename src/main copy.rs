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


#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();
    println!("Server starting....{}","Starting");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");


    let answers_dao = AnswersDaoImpl::new(pool);
    println!("Started");
    rocket::build()
        .mount(
            "/",
            routes![
                create_answer,
                get_answers,
            
            ],
        )
        .attach(CORS)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)

    
}

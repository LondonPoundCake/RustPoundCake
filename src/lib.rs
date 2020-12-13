pub mod schema;
pub mod models;

use self::models::{Todo,NewTodo};

#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::types::Timestamp;
use dotenv::dotenv;

use std::env;
use std::time::SystemTime;

pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}",database_url))
}

pub fn create_todo<'a>(conn: &PgConnection,title: &'a str,comment :&'a str,created :&'a str) -> Todo {

    use schema::todo;

    let new_todo = NewTodo {
        title : title,
        comment : comment,
        created : created
    };

    diesel::insert_into(todo::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new post")
}

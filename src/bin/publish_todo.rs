extern crate RustTODOproj;
extern crate diesel;

use self::diesel::prelude::*;
use self::RustTODOproj::*;
use self::models::Todo;
use std::env::args;

fn main() {
    use RustTODOproj::schema::todo::dsl::{todo,published};

    let id = args().nth(1).expect("publish_todo require a todo id")
        .parse::<i32>().expect("Invalid ID");
    
    let conn = establish_connection();

    let todo_test = diesel::update(todo.find(id))
                .set(published.eq(true))
                .get_result::<Todo>(&conn)
                .expect(&format!("Unable to find todo  {}", id));
    
    println!("Pushed todo {} ",todo_test.title);
}
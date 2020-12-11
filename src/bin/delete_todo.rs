extern crate RustTODOproj;
extern crate diesel;

use self::diesel::prelude::*;
use self::RustTODOproj::*;
use std::env::args;

fn main(){
    use RustTODOproj::schema::todo::dsl::*;

    let target = args().nth(1).expect("expect a target to match against");
    let pattern = format!("%{}%",target);

    let conn = establish_connection();
    let num_deleted = diesel::delete(todo.filter(title.like(pattern)))
        .execute(&conn)
        .expect("Error deleting todo");

    println!("Delete {} todo",num_deleted);
}
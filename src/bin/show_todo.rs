extern crate RustTODOproj;
extern crate diesel;

use self::RustTODOproj::*;
use self::models::*;
use self::diesel::prelude::*;

use std::collections::HashMap;


fn main (){
    use RustTODOproj::schema::todo::dsl::*;

    let con = establish_connection();
    let results = todo.filter(published.eq(true))
        .limit(5)
        .load::<Todo>(&con)
        .expect("Error loading posts");
    
    if results.len() != 0 {
        for Todo in results {
            print_todo(Todo);
        }
    } else {
        println!("No post's ");
    }
}

fn print_todo(todo_obj : Todo)
{
    let mut map = HashMap::new();
    map.insert("ID",(todo_obj.id).to_string());
    map.insert("TITLE",todo_obj.title);
    map.insert("BODY",todo_obj.body);
    map.insert("CREATED",todo_obj.created);
    println!("**********************************");
    for (key,val) in map.iter(){
        println!("{} => {}",key,val);
    }
    println!("**********************************");

}

extern crate RustTODOproj;
extern crate diesel;

use self::RustTODOproj::*;
use std::io::{stdin,Read};

use chrono::{Utc};




fn todo_timestamp_func() -> String {
    let todo_date = Utc::now().to_string();
    return todo_date;
}

fn todo_title_func() -> String {

    let mut todo_title = String::new();
    stdin().read_line(&mut todo_title).unwrap();
    let todo_title = &todo_title[..(todo_title.len() - 1)];
    return todo_title.to_string();
}

fn todo_comment_func() -> String {
    let mut comment = String::new();
    stdin().read_to_string(&mut comment).unwrap();
    return comment;
}

fn main() {
    let con = establish_connection();

    println!("Title ? ");
    
    let title = todo_title_func();

    println!("\nOK! Lets Write {} (Press {} when finished )\n",title,EOF);
    
    let todo = create_todo(&con,&title,&todo_comment_func(),&todo_timestamp_func());
    
    println!("\nSaved draft {} with id  {}",title,todo.id);
}

const EOF: &'static str = "CTRL+D";
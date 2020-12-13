extern crate RustTODOproj;
extern crate diesel;

use self::RustTODOproj::*;
use std::io::{stdin,Read};

use chrono::{Datelike,Timelike,Utc};




fn main() {
    let con = establish_connection();
    /* randon time things */
    let now = Utc::now();  
    let date_as_string = Utc::now().to_string();

    println!("Title ? ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("\nOK! Lets Write {} (Press {} when finished )\n",title,EOF);
    let mut comment = String::new();
    stdin().read_to_string(&mut comment).unwrap();

    let todo = create_todo(&con,title,&comment,&date_as_string);
    println!("\nSaved draft {} with id  {}",title,todo.id);
}

const EOF: &'static str = "CTRL+D";
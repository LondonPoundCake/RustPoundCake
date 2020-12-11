extern crate RustTODOproj;
extern crate diesel;

use self::RustTODOproj::*;
use std::io::{stdin,Read};

fn main() {
    let con = establish_connection();
    
    println!("Title ? ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("\nOK! Lets Write {} (Press {} when finished )\n",title,EOF);
    let mut comment = String::new();
    stdin().read_to_string(&mut comment).unwrap();

    let todo = create_todo(&con,title,&comment);
    println!("\nSaved draft {} with id  {}",title,todo.id);

}

const EOF: &'static str = "CTRL+D";
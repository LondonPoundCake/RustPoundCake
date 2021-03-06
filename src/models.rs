use super::schema::todo;

#[derive(Queryable)]
pub struct Todo {
    pub id : i32,
    pub title : String,
    pub body : String,
    pub published : bool
}

#[derive(Insertable)]
#[table_name="todo"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub comment: &'a str,
}
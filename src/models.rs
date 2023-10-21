use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub summary: String,
    pub message: String,
    pub done: bool
}

#[derive(Debug)]
pub struct Todos {
    pub todos: Vec<Todo>
}

impl Todos {
    pub fn new() -> Self {
        match fs::read_to_string("./todos.json") {
            Ok(data) => {
                let todos = serde_json::from_str(data.as_str())
                    .expect("Could not convert todos.json. Please make sure it is formatted correctly.");
                Todos { todos }
            },
            Err(_) => {
                Todos {
                    todos: vec![]
                }
            }
        }

    }

    fn save(&mut self) -> Result<(), io::Error> {
        fs::write(
               "./todos.json",
               json!(self.todos).to_string()
        )
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), io::Error> {
        self.todos.push(todo);

        self.save()
    }
}

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, io};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub summary: String,
    pub description: Option<String>,
    pub done: bool,
    pub id: Uuid
}

#[derive(Debug)]
pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        match fs::read_to_string("./todos.json") {
            Ok(data) => {
                let todos = serde_json::from_str(data.as_str()).expect(
                    "Could not convert todos.json. Please make sure it is formatted correctly.",
                );
                Todos { todos }
            }
            Err(_) => Todos { todos: vec![] },
        }
    }

    fn save(&mut self) -> Result<(), io::Error> {
        fs::write("./todos.json", json!(self.todos).to_string())
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), io::Error> {
        self.todos.push(todo);

        self.save()
    }

    pub fn done(&mut self, partial_id: &String) -> Result<(), io::Error> {
        self.todos
            .iter_mut()
            .filter(|todo| todo.id.to_string().starts_with(partial_id))
            .nth(0)
            .expect("Could not find todo starting with that ID.")
            .done = true;

        self.save()
    }
}

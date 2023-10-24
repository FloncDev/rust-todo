use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, io, env};
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
    location: String
}

impl Todos {
    pub fn new() -> Self {
        let location = env::var("HOME").expect("Sorry! This program only works on linux.") + "/.todos.json";

        match fs::read_to_string(&location) {
            Ok(data) => {
                let todos = serde_json::from_str(data.as_str()).expect(
                    "Could not convert todos.json. Please make sure it is formatted correctly.",
                );
                Todos { todos, location }
            }
            Err(_) => Todos { todos: vec![], location },
        }
    }

    fn save(&mut self) -> Result<(), io::Error> {
        fs::write(&self.location, json!(self.todos).to_string())
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), io::Error> {
        self.todos.push(todo);

        self.save()
    }

    fn todo_from_partial(&mut self, partial_id: &String) -> Option<&mut Todo> {
        self.todos
            .iter_mut()
            .filter(|todo| todo.id.to_string().starts_with(partial_id))
            .nth(0)
    }

    pub fn done(&mut self, partial_id: &String) -> Result<(), io::Error> {
        self.todo_from_partial(partial_id)
            .expect("Could not find todo starting with that ID.")
            .done = true;

        self.save()
    }

    pub fn rm(&mut self, partial_id: &String) -> Result<(), io::Error> {
        let todo_id = self.todo_from_partial(partial_id)
            .expect("Could not find todo starting with that ID.")
            .id;

        let index = self.todos
            .iter()
            .position(|x| x.id == todo_id)
            .unwrap();

        self.todos.remove(index);

        self.save()
    }
}

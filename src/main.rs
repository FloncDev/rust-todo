use clap::{value_parser, Arg, ArgAction, Command};
use crate::models::{Todo, Todos};
use uuid::Uuid;
use colored::Colorize;

pub mod models;

fn cli() -> Command {
    Command::new("td")
        .about("A simple todo app written in rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("List all todos. -a will also show completed todos.")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .required(false)
                ),
        )
        .subcommand(
            Command::new("new").about("Create a new todo").arg(
                Arg::new("summary")
                    .short('s')
                    .long("summary")
                    .value_parser(value_parser!(String))
                    .required(true)
            )
            .arg(
                Arg::new("description")
                .short('d')
                .long("desc")
                .value_parser(value_parser!(String)),
            )
        )
        .subcommand(
            Command::new("done").about("Toggle done on a todo").arg(
                Arg::new("id")
                    .value_parser(value_parser!(String))
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("rm").about("Remove a todo").arg(
                Arg::new("id")
                    .value_parser(value_parser!(String))
                    .required(true),
            ),
        )
}

fn main() {
    let matches = cli().get_matches();

    let mut todos = Todos::new();

    match matches.subcommand() {
        Some(("new", cmd_match)) => {
            todos.add(Todo {
                summary: cmd_match.get_one::<String>("summary").unwrap().to_owned(),
                description: cmd_match.get_one::<String>("description").cloned(),
                done: false,
                id: Uuid::new_v4(),
            }).unwrap();
        },
        Some(("list", cmd_match)) => {
            let user_todos =
                todos.todos
                .iter()
                .filter(
                    |todo|
                    !todo.done ||
                    todo.done == cmd_match.get_flag("all")
                );
            
            for user_todo in user_todos {
                let done = if &user_todo.done == &true { "DONE".green() } else { "".normal() };
                let description = user_todo.clone().description.unwrap_or(String::from(""));

                println!(
                    "\n{}\n",
                    format!(
                        "{}\n{} {}\n{}",
                        format!("{}", user_todo.id).bright_black(),
                        user_todo.summary.bold(),
                        done,
                        description
                    ).trim()
                );
            }
        },
        Some(("done", cmd_match)) => {
            let partial_id = cmd_match.get_one::<String>("id").unwrap();
            todos.done(partial_id).unwrap();
        },
        Some(("rm", cmd_match)) => {
            let partial_id = cmd_match.get_one::<String>("id").unwrap();
            todos.rm(partial_id).unwrap();
        },
        _ => {println!("other")}
    }
}

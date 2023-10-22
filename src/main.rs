use clap::{value_parser, Arg, ArgAction, Command};

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
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("new").about("Create a new todo").arg(
                Arg::new("summary")
                    .short('s')
                    .long("summary")
                    .value_parser(value_parser!(String))
                    .required(true)
            ),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("desc")
                .value_parser(value_parser!(String)),
        )
        .subcommand(
            Command::new("done").about("Toggle done on a todo").arg(
                Arg::new("id")
                    .value_parser(value_parser!(u32))
                    .required(true),
            ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", cmd_match)) => {
            println!("{:#?}", cmd_match.get_one::<String>("summary").unwrap());
        },
        _ => {println!("other")}
    }
}

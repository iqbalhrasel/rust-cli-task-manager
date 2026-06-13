use clap::{Arg, Command};

use crate::task_service::TaskService;

mod clap_builder;
mod db;
mod model;
mod task_service;

fn main() {
    let _conn = match db::init() {
        Ok(conn) => {
            let task_service = TaskService::new(conn);
            clap_builder::cli_process(task_service);
        }
        Err(e) => {
            println!("error: {:?}", e)
        }
    };
}

fn clap_subcommand() {
    //cargo run -- add -n alice -a 22
    // myapp add -n alice -a 22

    let matches = Command::new("myapp")
        .subcommand(
            Command::new("add")
                .arg(Arg::new("name").long("name").short('n').required(true))
                .arg(Arg::new("age").long("age").short('a').required(true)),
        )
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let name = add_matches.get_one::<String>("name").unwrap();
        let age = add_matches
            .get_one::<String>("age")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        println!("{name} {age}")
    }
}

fn clap_exmpl() {
    //cargo run -- --name ihr
    let matches = Command::new("myapp")
        .version("1.0")
        .about("greet people")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .required(true)
                .help("user name"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .default_value("1"),
        )
        .get_matches();
    let name = matches.get_one::<String>("name").unwrap();
    let count = matches
        .get_one::<String>("count")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    println!("{name} {count}")
}

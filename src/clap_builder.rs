use clap::{Arg, ArgMatches, Command};

use crate::task_service::TaskService;

pub fn cli_process(task_service: TaskService) {
    // cargo run -- add "attend meeting"
    // cargo run -- del 2
    // cargo run -- done --id 1 --done y
    // cargo run -- upd --id 1 --desc "meeting of rust"
    // cargo run -- all
    let matches = Command::new("dtm")
        .subcommand(
            Command::new("add").arg(Arg::new("desc").required(true).help("task description")),
        )
        .subcommand(Command::new("del").arg(Arg::new("id").required(true).help("task id")))
        .subcommand(
            Command::new("done")
                .arg(
                    Arg::new("id")
                        .long("id")
                        .short('i')
                        .required(true)
                        .help("task id"),
                )
                .arg(
                    Arg::new("done")
                        .long("done")
                        .short('d')
                        .required(true)
                        .help("n/y"),
                ),
        )
        .subcommand(
            Command::new("upd")
                .arg(
                    Arg::new("id")
                        .long("id")
                        .short('i')
                        .required(true)
                        .help("task id"),
                )
                .arg(
                    Arg::new("desc")
                        .long("desc")
                        .short('d')
                        .required(true)
                        .help("task description"),
                ),
        )
        .subcommand(Command::new("list"))
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_task(add_matches, &task_service);
    }

    if let Some(del_matches) = matches.subcommand_matches("del") {
        delete_task(del_matches, &task_service);
    }

    if let Some(done_matches) = matches.subcommand_matches("done") {
        done_task(done_matches, &task_service);
    }

    if let Some(upd_matches) = matches.subcommand_matches("upd") {
        update_desc(upd_matches, &task_service);
    }

    if let Some(_all_matches) = matches.subcommand_matches("list") {
        get_all_task(&task_service);
    }
}

fn update_desc(matches: &ArgMatches, task_service: &TaskService) {
    let id = matches
        .get_one::<String>("id")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let desc = matches.get_one::<String>("desc").unwrap();

    match task_service.update_desc(id, desc) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn delete_task(matches: &ArgMatches, task_service: &TaskService) {
    let id = matches
        .get_one::<String>("id")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    match task_service.delete(id) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn get_all_task(task_service: &TaskService) {
    match task_service.get_all() {
        Err(e) => println!("{}", e),
        Ok(tasks) => {
            println!("------------ Your Tasks ------------");
            for t in tasks {
                let done = if t.done { "done" } else { "not done" };
                println!("[{}] {} -- {}", t.id, done, t.description)
            }
            println!("--------------- End ----------------");
        }
    }
}

fn add_task(matches: &ArgMatches, task_service: &TaskService) {
    let desc = matches.get_one::<String>("desc").unwrap();
    match task_service.add(desc) {
        Err(e) => println!("{}", e),
        Ok(_) => {}
    }
}

fn done_task(matches: &ArgMatches, task_service: &TaskService) {
    let id = matches
        .get_one::<String>("id")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let done_cmd = matches.get_one::<String>("done").unwrap();

    let done = match done_cmd.to_lowercase().chars().next() {
        Some('y') => true,
        Some('n') => false,
        _ => {
            println!("incorrect command. use y or n");
            return;
        }
    };

    match task_service.done_task(id, done) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}

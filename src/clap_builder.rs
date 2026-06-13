use clap::{Arg, Command};

pub fn cli_process() {
    //add str
    // del id
    // done id
    // all

    // cargo run -- add "attend meeting"
    // cargo run -- del 2
    // cargo run -- done --id 1 --done y
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
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let desc = add_matches.get_one::<String>("desc").unwrap();

        println!("{desc}")
    }

    if let Some(del_matches) = matches.subcommand_matches("del") {
        let id = del_matches
            .get_one::<String>("id")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        println!("{id}")
    }

    if let Some(done_matches) = matches.subcommand_matches("done") {
        let id = done_matches
            .get_one::<String>("id")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let done = done_matches.get_one::<String>("done").unwrap();

        println!("{id} {done}")
    }
}

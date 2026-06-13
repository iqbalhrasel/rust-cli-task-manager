use clap::{Arg, Command};

pub fn cli_process() {
    //add str
    // del id
    // done id
    // all

    // cargo run -- add "attend meeting"
    let matches = Command::new("dtm")
        .subcommand(
            Command::new("add").arg(Arg::new("desc").required(true).help("task description")),
        )
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let desc = add_matches.get_one::<String>("desc").unwrap();

        println!("{desc}")
    }
}

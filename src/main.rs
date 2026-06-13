use clap::{Arg, Command};

fn main() {
    clap_exmpl();
}

fn clap_subcommand() {}

fn clap_exmpl() {
    //cargo run -- --name ihr
    let matches = Command::new("add")
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

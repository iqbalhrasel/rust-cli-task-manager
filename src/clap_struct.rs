use clap::{Parser, Subcommand};

use crate::task_service::TaskService;

#[derive(Subcommand)]
enum Commands {
    Add {
        desc: String,
    },
    Del {
        id: i64,
    },

    Done {
        #[arg(short, long)]
        id: i64,

        #[arg(short, long)]
        done: String,
    },

    Upd {
        #[arg(short, long)]
        id: i64,

        #[arg(short, long)]
        desc: String,
    },

    List,
}

#[derive(Parser)]
#[command(name = "dtm")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub fn cli_process_st(task_service: TaskService) {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { desc } => {
            add_task(&task_service, &desc);
        }

        Commands::Del { id } => {
            delete_task(&task_service, id);
        }

        Commands::Done { id, done } => {
            done_task(&task_service, id, &done);
        }

        Commands::Upd { id, desc } => {
            update_desc(&task_service, id, &desc);
        }

        Commands::List => {
            get_all_task(&task_service);
        }
    }
}

fn add_task(task_service: &TaskService, desc: &str) {
    if let Err(e) = task_service.add(desc) {
        println!("{}", e);
    }
}

fn delete_task(task_service: &TaskService, id: i64) {
    if let Err(e) = task_service.delete(id) {
        println!("{}", e);
    }
}

fn update_desc(task_service: &TaskService, id: i64, desc: &str) {
    if let Err(e) = task_service.update_desc(id, desc) {
        println!("{}", e);
    }
}

fn get_all_task(task_service: &TaskService) {
    match task_service.get_all() {
        Err(e) => println!("{}", e),

        Ok(tasks) => {
            println!("------------ Your Tasks ------------");

            for t in tasks {
                let done = if t.done { "done" } else { "not done" };

                println!("[{}] {} -- {}", t.id, done, t.description);
            }

            println!("--------------- End ----------------");
        }
    }
}

fn done_task(task_service: &TaskService, id: i64, done_cmd: &str) {
    let done = match done_cmd.to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("incorrect command. use y or n");
            return;
        }
    };

    if let Err(e) = task_service.done_task(id, done) {
        println!("{}", e);
    }
}

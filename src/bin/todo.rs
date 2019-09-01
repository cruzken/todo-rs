use std::env;
use todo_rs::db::{create_task, establish_connection};

fn help() {
    println!("subcommands");
    println!("    new<title>: create a new task");
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        _ => help(),
        
    }
}

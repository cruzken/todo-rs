use std::env;
use todo_rs::db::{done_update_task, query_task, create_task, establish_connection};

fn help() {
    println!("subcommands");
    println!("    new<title>: create a new task");
    println!("    show: show all tasks");
    println!("    done<id>: mark task done");
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

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        let status = match task.done {
            0 => "Pending",
            1 => "Done",
            _ => "Unknown",
        };
        println!("{}. {} - {}", task.id, task.title, status);
    }
}

fn done_task(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing argument");
        help();
        return;
    }

    let id = &args[0]
        .parse::<i32>().expect("Invalid ID");

    let conn = establish_connection();
    done_update_task(&conn, *id);
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
        "show" => show_tasks(&args[2..]),
        "done" => done_task(&args[2..]),
        _ => help(),
        
    }
}

use std::env;
use todo_rs::db::{create_task, del_task, done_update_task, establish_connection, query_task};

fn help() {
    println!("subcommands");
    println!("    new<title, user>: create a new task");
    println!("    show<user>: show all tasks from user");
    println!("    done<id>: mark task done");
    println!("    delete<id>: delete task");
}

fn new_task(args: &[String]) {
    if args.len() < 2 {
        println!("new: missing <title, user>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0], &args[1]).unwrap();
}

fn show_tasks(args: &[String]) {
    if args.len() < 1 {
        println!("show: missing<user>");
        return;
    }

    if args.len() > 1 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let user = &args[0];

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn, user).unwrap() {
        let status = match task.done {
            0 => "Pending",
            1 => "Done",
            _ => "Unknown",
        };
        println!("{}. {} {} - {}", task.id, task.user, task.title, status);
    }
}

fn done_task(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing argument");
        help();
        return;
    }

    let id = &args[0].parse::<i32>().expect("Invalid ID");

    let conn = establish_connection();
    done_update_task(&conn, *id).unwrap();
}

fn delete_task(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing argument");
        help();
        return;
    }

    let id = &args[0].parse::<i32>().expect("Invalid ID");

    let conn = establish_connection();
    del_task(&conn, *id).unwrap();
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
        "delete" => delete_task(&args[2..]),
        _ => help(),
    }
}

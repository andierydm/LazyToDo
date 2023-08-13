use clap::Parser;
use crate::lazy_todo::{LazyToDo, ToDoTrait};

mod lazy_todo;

#[derive(Parser, Debug)]
#[command(version = "1.0")]
#[command(about = "Very simple to do list. Without arguments, print all not done entries", long_about = None)]
struct Args {
    ///Insert a new entry
    #[arg(short, long, value_name = "CONTENT")]
    insert: Option<String>,

    ///Print entries in full format
    #[arg(short, long)]
    full: bool,

    ///Print only completed entries
    #[arg(short, long)]
    done: bool,

    ///Mark a entry as completed
    #[arg(long, short, value_name = "ID")]
    mark: Option<i64>,
}

fn print_entries(todo: LazyToDo, full: bool, checked: bool) {
    let entries = match todo.get_all(checked) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    if entries.len() == 0 {
        println!("No entries");
        std::process::exit(0);
    }

    for x in entries {
        let is_done = if x.done {
            "[*]"
        } else {
            "[]"
        };

        if full {
            println!("{} {} - {}", is_done, x.id, x.description);
            println!("\t{}", x.create_date);
            println!()
        } else {
            println!("{} {}", is_done, x.description);
        }
    }
}

fn main() {
    let todo = match LazyToDo::new() {
        Ok(todo) => todo,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let args = Args::parse();

    if let Some(content) = args.insert {
        match todo.insert(content) {
            Ok(_) => println!("Done!"),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    if let Some(id) = args.mark {
        match todo.mark_as_done(id) {
            Ok(_) => println!("Done!"),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    print_entries(todo, args.full, args.done);
}

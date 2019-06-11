//! Simple CLI TODO REPL app

mod todo;

use std::io;
use std::io::Write;

use todo::{Todo, Id};

const TASKS_PATH: &str = "tasks.json";

fn main() -> io::Result<()> {
    let mut todo = Todo::load(TASKS_PATH).unwrap_or_default();

    todo.print();
    loop {
        print!("> ");
        io::stdout().flush()?; // force output

        let input = &mut String::new();
        io::stdin().read_line(input)?;

        let command = input.trim().splitn(2, ' ').collect::<Vec<_>>();
        match command.as_slice() {
            ["add", title] => {
                todo.add(title);
                todo.print();
            }
            ["remove", id] => {
                if let Some(id) = parse_id(id) {
                    todo.remove(id);
                    todo.print();
                }
            }
            ["done", id] => {
                if let Some(id) = parse_id(id) {
                    todo.mark_as_completed(id);
                    todo.print();
                }
            }
            ["undo", id] => {
                if let Some(id) = parse_id(id) {
                    todo.mark_as_uncompleted(id);
                    todo.print();
                }
            }
            ["exit"] | ["quit"] | ["bye"] => {
                println!("Bye!");
                break;
            }
            unknown => {
                println!("Unknown command: {:?}", unknown);
                usage();
            }
        }
    }

    todo.save(TASKS_PATH)
}

/// Parse Id from the string. Id is an index, so it's whatever number
/// is parsed from the string minus one.
fn parse_id(s: &str) -> Option<Id> {
    match s.parse::<Id>() {
        Ok(id) => Some(id - 1),
        Err(err) => {
            eprintln!("Error parsing NUMBER: {}", err);
            None
        }
    }
}

fn usage() {
    println!("Usage:");
    println!("  list [all|active|completed]");
    println!("  add ITEM");
    println!("  remove NUMBER");
    println!("  done NUMBER");
    println!("  undo NUMBER");
    println!("  help");
    println!("  exit|quit|bye");
}

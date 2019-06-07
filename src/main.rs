//! Simple CLI TODO REPL app

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct Task {
    title: String,
    completed: bool,
}

#[derive(Deserialize, Serialize, Default)]
struct Todo {
    tasks: Vec<Task>,
}

type Id = usize;

impl Todo {
    fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        serde_json::from_reader(file).map_err(|err| {
            eprintln!("Cannot deserialize from {:?}: {}", path, err);
            io::ErrorKind::InvalidData.into()
        })
    }

    fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        serde_json::to_writer(file, self).map_err(|err| {
            eprintln!("Cannot serialize to {:?}: {}", path, err);
            io::ErrorKind::InvalidInput.into()
        })
    }

    fn print(&self) {
        println!("{}", self);
    }

    fn add(&mut self, title: impl ToString) {
        self.tasks.push(Task {
            title: title.to_string(),
            completed: false,
        });
    }

    fn mark_as_completed(&mut self, id: Id) {
        self.tasks.get_mut(id).map(|todo| todo.completed = true);
    }

    fn mark_as_imcompleted(&mut self, id: Id) {
        self.tasks.get_mut(id).map(|todo| todo.completed = false);
    }

    fn remove(&mut self, id: Id) {
        if id < self.tasks.len() {
            self.tasks.remove(id);
        } else {
            eprintln!("No such item: {}", id);
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, todo) in self.tasks.iter().enumerate() {
            let status = if todo.completed { " DONE" } else { "" };
            write!(f, "{}){} {}\n", i + 1, status, todo.title)?;
        }
        write!(f, "Total: {}", self.tasks.len())
    }
}

fn main() -> io::Result<()> {
    let tasks_path = "tasks.json";
    let mut todo = Todo::load(tasks_path).unwrap_or_default();

    todo.print();
    loop {
        print!("> ");
        io::stdout().flush()?; // force output

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            continue;
        }
        let command = input.trim().splitn(2, ' ').collect::<Vec<_>>();
        match command[0] {
            "add" => {
                todo.add(command[1]);
                todo.print();
            }
            "remove" => {
                parse_id(command[1]).map(|id| {
                    todo.remove(id);
                    todo.print();
                });
            }
            "done" => {
                parse_id(command[1]).map(|id| {
                    todo.mark_as_completed(id);
                    todo.print();
                });
            }
            "undo" => {
                parse_id(command[1]).map(|id| {
                    todo.mark_as_imcompleted(id);
                    todo.print();
                });
            }
            "exit" | "quit" | "bye" => {
                println!("Bye!");
                break;
            }
            unknown => {
                println!("Unknown command: {}", unknown);
                usage();
            }
        }
    }

    todo.save(tasks_path)
}

fn parse_id(s: &str) -> Option<usize> {
    match s.parse::<usize>() {
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

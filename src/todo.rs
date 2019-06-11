use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct Task {
    title: String,
    completed: bool,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Todo {
    tasks: Vec<Task>,
}

pub type Id = usize;

impl Todo {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        serde_json::from_reader(file).map_err(|err| {
            eprintln!("Cannot deserialize from {:?}: {}", path, err);
            io::ErrorKind::InvalidData.into()
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        serde_json::to_writer(file, self).map_err(|err| {
            eprintln!("Cannot serialize to {:?}: {}", path, err);
            io::ErrorKind::InvalidInput.into()
        })
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn add(&mut self, title: impl ToString) {
        self.tasks.push(Task {
            title: title.to_string(),
            completed: false,
        });
    }

    pub fn mark_as_completed(&mut self, id: Id) {
        self.tasks.get_mut(id).map(|todo| todo.completed = true);
    }

    pub fn mark_as_uncompleted(&mut self, id: Id) {
        self.tasks.get_mut(id).map(|todo| todo.completed = false);
    }

    pub fn remove(&mut self, id: Id) {
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

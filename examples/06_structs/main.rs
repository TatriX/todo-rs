struct Task {
    title: String,
    completed: bool,
}

fn main() {
    let tasks = vec![
        Task{title: "Learn Rust".to_string(), completed: false},
        Task{title: "Be cool".to_string(), completed: false},
    ];

    for task in tasks {
        println!("Title: {}\n  Completed: {}", task.title, task.completed);
    }
}

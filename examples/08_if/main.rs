struct Task {
    title: String,
    completed: bool,
}

fn main() {
    let tasks = vec![
        Task{title: "Learn Rust".to_string(), completed: false},
        Task{title: "Be cool".to_string(), completed: true},
    ];

    for task in &tasks {
        println!("Title: {}", task.title);
        if task.completed {
            println!("  Completed!");
        }
    }

    println!();

    for task in &tasks {
        let status = if task.completed {
            "[X]"
        } else {
            "[ ]"
        };
        println!("{} {}", status, task.title);
    }
}

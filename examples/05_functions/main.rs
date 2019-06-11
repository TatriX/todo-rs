fn main() {
    let tasks = vec!["Learn rust", "Be cool"];
    print_tasks(tasks);
}

fn print_tasks(tasks: Vec<&str>) {
    for task in &tasks {
        println!("{}", task);
    }
    println!("Total: {}", tasks.len());
}

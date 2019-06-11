fn main() {
    let mut tasks = Vec::new();
    tasks.push("Learn Rust");
    tasks.push("Be cool");

    println!("Tasks: {:?}", tasks);

    tasks.push("Be awesome");
    tasks.remove(1);

    println!("Tasks: {:?}", tasks);
}

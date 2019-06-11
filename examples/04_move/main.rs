fn main() {
    let mut tasks = vec!["Learn rust", "Be cool"];

    for task in &tasks {
        println!("{}", task)
    }
    // same as
    for task in tasks.iter() {
        println!("{}", task)
    }

    println!(); // print empty line

    tasks.push("Exercise!");

    for (i, task) in tasks.iter().enumerate() {
        println!("{}) {}", i, task);
    }
}

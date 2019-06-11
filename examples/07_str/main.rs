fn main() {
    let s: &str = "I'm str";
    let s1: String = "I'm String".to_string();
    let s2: String = String::from("I'm String");
    let s3: String = "I'm String".into(); // try removing type
    let mut s4: String = String::new();
    s4 += s;
    let s5: String = format!("I'm {}", "string");
    println!("{} {} {} {} {} {}", s, s1, s2, s3, s4, s5);
}

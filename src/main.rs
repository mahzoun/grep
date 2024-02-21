use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    let program_name = &args[0];
    println!("Program name is {}", program_name);
    println!("Query is {}", query);
    println!("File path is {}", file_path);
}

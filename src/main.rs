use std::{env,fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    let program_name = &args[0];
    println!("Program name is {}", program_name);
    println!("Query is {}", query);
    println!("File path is {}", file_path);
    let content = fs::read_to_string(file_path).expect("Error in opening the file");
    println!("with text:\n{content}");
}

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching {}", query);
    println!("In file {}", file_path);

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the content of the file");
    println!(
        " THE CONTENT OF THE FILE IS \n =================================\n \n  {}",
        content
    );
}

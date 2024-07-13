use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in {}", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file at {}", file_path).as_str());

    println!("With text:\n{}", contents);
}

use std::fs;

fn main() {
    let file_path = "test_file";
    let file_contents: String = fs::read_to_string(file_path).expect("Unable to read the file");
    println!("{}",file_contents);
  }

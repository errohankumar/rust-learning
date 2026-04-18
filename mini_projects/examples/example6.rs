use std::fs::read_to_string;

fn main() {
    match read_from_file_rohan(String::from("a.txt")) {
        Ok(data) => println!("File contents:\n{}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_from_file_rohan(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("File not read: {}", err)),
    }
}
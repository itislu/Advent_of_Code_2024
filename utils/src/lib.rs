pub mod input {
    use std::env;
    use std::fs;
    use std::path;

    fn read_file(filename: &str) -> String {
        let dir = match env::var("CARGO_MANIFEST_DIR") {
            Ok(dir) => path::PathBuf::from(dir),
            Err(_) => env::current_dir().expect("Failed to get current directory"),
        };
        let path = dir.join(filename);
        fs::read_to_string(&path).expect(&format!("Failed to read file {}", path.display()))
    }

    pub fn read_input() -> String {
        read_file("input.txt")
    }

    pub fn read_example() -> String {
        read_file("input_example.txt")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::read_example;
    use input::read_input;

    #[test]
    fn test_read_input() {
        let result = read_input();
        println!("{}", result);
    }

    #[test]
    fn test_read_example() {
        let result = read_example();
        println!("{}", result);
    }
}

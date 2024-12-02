pub mod input {
    use std::env;
    use std::fs;
    use std::path;

    fn read_file(filename: &str) -> String {
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path_str = format!("{}/{}", dir, filename);
        let path = path::Path::new(&path_str);
        fs::read_to_string(path).expect(&format!("Failed to read file {}", path.display()))
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

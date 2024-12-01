pub mod input {
    use std::fs;
    use std::path;

    pub fn read_file(day: &str) -> String {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path_str = format!("{}/../inputs/{}", root, day);
        let path = path::Path::new(&path_str);
        fs::read_to_string(path).expect(&format!("Failed to read file {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::read_file;

    #[test]
    fn test_read_file() {
        let result = read_file("day01_example");
        println!("{}", result);
    }
}

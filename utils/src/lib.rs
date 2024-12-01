pub mod input {
    use std::fs;
    use std::path;

    pub fn read_file(day: u32, test: bool) -> String {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path_str = format!(
            "{}/../inputs/day{:02}{}",
            root,
            day,
            if test { "_test" } else { "" }
        );
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
        let result = read_file(1, true);
        println!("{}", result);
    }
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn read_file(fileuri: String) -> String {
    let path = Path::new(&fileuri);

    // Try open file. If no file found return empty string.
    let mut f = match File::open(path) {
        Err(_why) => return String::new(),
        Ok(f) => f,
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    return contents;
}

fn save_file(fileuri: String, content: String) -> i8 {
    return 1;
}

fn main() {
    let fileuri = "./src/test.txt".to_string();
    let content = read_file(fileuri);
    println!("{}", content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let fileuri = "./src/test.txt".to_string();
        let content = "test".to_string();
        assert_eq!(read_file(fileuri), content);
    }

    #[test]
    fn test_save_file() {
        let fileuri = "./src/test2.txt";
        let content = "test";
        assert_eq!(save_file(fileuri.to_string(), content.to_string()), 0);
        assert_eq!(read_file(fileuri.to_string()), content.to_string());

    }

}
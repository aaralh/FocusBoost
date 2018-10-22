use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(fileuri: String) -> String {
    return fileuri;
}


fn main() {
    let fileuri = "./src/test.txt".to_string();
    let content = read_file(fileuri);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let fileuri = "./src/text.txt";
        let content = "test".to_string();
        assert_eq!(read_file(fileuri.to_string()), content);
    }

}
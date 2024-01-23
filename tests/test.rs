#[cfg(test)]
mod tests {
    use super::load_file;
    use std::fs::File;
    use std::io::Write;


    #[test]
    fn test_load_file() {
        // Create a temporary directory
        let file_path = "./tests/test1.json";

        // Write some JSON content to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"key": {{"value": "value1", "class": "class1"}}}}"#).unwrap();

        // Call load_file function
        let result = load_file(file_path);

        // Check the result
        match result {
            Ok(dict) => {
                assert_eq!(dict.len(), 1);
                let envelope = dict.get("key").unwrap();
                assert_eq!(envelope.value, "value1");
                assert_eq!(envelope.class, "class1");
            },
            Err(_) => panic!("load_file failed"),
        }

        // Close the temporary directory
        dir.close().unwrap();
    }
}
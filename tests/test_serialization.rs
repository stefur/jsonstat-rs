
#[cfg(test)]
mod tests {
    use serde_json;
    use std::fs;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use jsonstat::*; 
    use std::path::Path;

    fn read_json_file(file_path: &Path) -> Result<String, std::io::Error> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut json_string = String::new();
        reader.read_to_string(&mut json_string)?;
        Ok(json_string)
    }
    
    #[test]
    fn test_roundtrip() {
        let json_files_dir = "tests/test_data/";
    
        for entry in fs::read_dir(json_files_dir).unwrap() {
            let entry = entry.unwrap();
            let file_path = &entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            // Read JSONStat file into a string
            let json_string = match read_json_file(&file_path) {
                Ok(json) => json,
                Err(e) => {
                    println!("Error reading {}: {}", file_name, e);
                    continue;
                }
            };

            // Deserialize JSONStat from the string
            let json_stat: JSONStat = match serde_json::from_str(&json_string) {
                Ok(json) => json,
                Err(e) => {
                    println!("Error deserializing {}: {}", file_name, e);
                    continue;
                }
            };

            // Serialize JSONStat again
           let serialized_json_stat = serde_json::to_string(&json_stat).unwrap();

            // Deserialize serialized JSON back into JSONStat
            let deserialized_json_stat: JSONStat = serde_json::from_str(&serialized_json_stat).unwrap();
    
            // Assert that the deserialized data still looks ok
            assert_eq!(
                json_stat, 
                deserialized_json_stat, 
                "we are making sure that deserialization works as expected for {}", 
                file_name
            );
        }
    }
    
}
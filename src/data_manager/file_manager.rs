pub mod file_manager {
    use std::path::Path;
    use std::fs::File;
    use std::fs;
    use serde_json;
    use crate::data_structure::projects::projects;

    // FileManager manages the file
    pub struct FileManager{
        path_str: String,
    }

    // new_file_manager creates a file manager
    pub fn new_file_manager(path_str: String) -> FileManager{
        FileManager{
            path_str: path_str,
        }
    }

    impl FileManager {
        // create_file creates the file
        pub fn create_file(&self){
            println!("Creating file: {}", self.path_str);
            let path = Path::new(&self.path_str);
            if !path.exists(){
                println!("File does not exist: {}", self.path_str);
                match File::create(path) {
                    Ok(file) => println!("Successfully created file"),
                    Err(e) => panic!("File creation failed due to{}", e)
                }
            }
        }

        // delete_file deletes the file
        pub fn delete_file(&self){
            println!("Deleting file: {}", self.path_str);
            let path = Path::new(&self.path_str);
            if path.exists(){
                match fs::remove_file(path){
                    Ok(_) => println!("Successfully deleted file"),
                    Err(e) => panic!("File delete failed due to{}", e)
                }
            }else{
                panic!("The file does not exist");
            }
        }

        // read_file_to_string reads the file
        pub fn read_file_to_string(&self) -> String{
            println!("Reading file {}...", self.path_str);
            let path = Path::new(&self.path_str);
            if path.exists(){
                match fs::read_to_string(path){
                    Ok(result) => result,
                    Err(e) => panic!("File delete failed due to{}", e)
                }
            }else{
                panic!("The file does not exist");
            }
        }

        // write_string_to_file writes a string to a file
        pub fn write_string_to_file(&self, content: String){
            println!("Writing to file {}", self.path_str);
            let path = Path::new(&self.path_str);
            if path.exists(){
                match fs::write(path, content.as_bytes()){
                    Ok(result) => {},
                    Err(e) => panic!("File delete failed due to{}", e)
                }
            }else{
                panic!("The file does not exist");
            }
        }

        // read_file_to_projects reads the file to Projects struct
        pub fn read_file_to_projects(&self) -> projects::Projects {
            println!("Reading file {}", self.path_str);
            let path = Path::new(&self.path_str);
            if path.exists(){
                match fs::read_to_string(path){
                    Ok(result) => {
                        let result: projects::Projects = serde_json::from_str(&result)
                            .expect("Could not deserialize projects from file");
                        result
                    },
                    Err(e) => panic!("File read failed due to{}", e)
                }
            }else{
                panic!("The file {} does not exist", self.path_str);
            }
        }

        // check_file_exist checks whether this file exists
        pub fn check_file_exist(&self) -> bool {
            let path = Path::new(&self.path_str);
            path.exists()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use defer_rs::defer;

    #[test]
    // Test create_file
    fn test_create_file(){
        let test_fm = file_manager::new_file_manager("test.json".to_string());
        test_fm.create_file();
        defer!({
            test_fm.delete_file();
        });
        let path = Path::new("test.json");
        if !path.exists() {
            panic!("File does not exist, creation failed");
        }
    }

    #[test]
    // Test write_file and read_file
    fn test_write_read_file(){
        let test_fm = file_manager::new_file_manager("test.txt".to_string());
        test_fm.create_file();
        defer!({
            test_fm.delete_file();
        });
        test_fm.write_string_to_file("hello world".to_string());
        let content = test_fm.read_file_to_string();
        assert_eq!("hello world", content);
    }
}

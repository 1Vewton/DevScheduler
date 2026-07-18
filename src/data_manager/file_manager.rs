pub mod file_manager {
    use std::path::Path;
    use std::fs::File;

    // FileManager manages the file
    pub struct FileManager{
        path_str: String,
    }

    impl FileManager {
        // Create file
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
    }
}

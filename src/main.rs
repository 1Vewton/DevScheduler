mod data_structure;
mod utils;
mod data_manager;
use std::io;
use crate::data_structure::projects::projects;
use crate::data_manager::file_manager::file_manager;

fn main() {
    let projects_manager = file_manager::new_file_manager(
        String::from("projects.json")
    );
    // Create file
    if !projects_manager.check_file_exist(){
        projects_manager.create_file();
        let mut user_name = String::new();
        println!("Please enter the user name");
        io::stdin().read_line(&mut user_name);
        let mut new_projects = projects::new_projects(user_name.trim().to_string());
        projects_manager.write_string_to_file(new_projects.to_string());
    }
    // Read projects
    let mut projects = projects_manager.read_file_to_projects();
    println!("Hello {}", projects.get_user_name());
    if projects.get_all_projects().is_empty() {
        println!("There is no project, you can add project now");
    }else{

    }
}

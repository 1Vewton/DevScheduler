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
    let project_today_manager = file_manager::new_file_manager(
        String::from("projects_today.json")
    );
    // Create file
    if !projects_manager.check_file_exist(){
        projects_manager.create_file();
        let mut user_name = String::new();
        println!("Please enter the user name");
        io::stdin().read_line(&mut user_name).expect("Failed to read user name");
        let mut new_projects = projects::new_projects(user_name.trim().to_string());
        projects_manager.write_string_to_file(new_projects.to_string());
    }
    // Read projects
    let mut projects = projects_manager.read_file_to_projects();
    println!("Hello {}", projects.get_user_name());
    // Get project for today
    if projects.get_all_projects().is_empty() {
        println!("There are no projects, you can add project now");
    }else {
        let mut project_today: projects::ProjectToday;
        if project_today_manager.check_file_exist() {
            project_today_manager.create_file();
            project_today = projects::new_project_today(projects);
            project_today_manager.write_string_to_file(project_today.to_string());
        } else {
            project_today = project_today_manager.read_file_to_project_today();
            if !project_today.is_valid() {
                project_today = projects::new_project_today(projects);
                project_today_manager.write_string_to_file(project_today.to_string());
            }
        }
    }
    // Operation here
    loop {
        println!("Please enter operation you prefer");
        println!("c: Create new project");
        println!("q: Close the program");
        let mut operation = String::new();
        // Read operation
        io::stdin().read_line(&mut operation).expect("Failed to read operation");
        match operation.as_str() {
            "c" => {

            },
            "q" => break,
            _ => {
                println!("Operation not supported");
            }
        }
    }
}

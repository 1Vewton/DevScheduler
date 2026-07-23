mod data_structure;
mod utils;
mod data_manager;
use std::io;
use crate::data_structure::projects::projects;
use crate::data_manager::file_manager::file_manager;
use crate::utils::question_interface::question_interface;

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
    if projects.clone().get_all_projects().is_empty() {
        println!("There are no projects, you can add project now");
    }else {
        let mut project_today: projects::ProjectToday;
        if !project_today_manager.check_file_exist() {
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
        match operation.as_str().trim() {
            "c" => {
                // Create project
                println!("Enter the name of the project");
                let mut project_name = String::new();
                io::stdin().read_line(&mut project_name).expect("Failed to read project_name");
                let mut weight = String::from("1");
                let weight_int: i64;
                println!("Input a weight for this project in integer (e.g. 1)");
                io::stdin().read_line(&mut weight).expect("Failed to read weight");
                match weight.trim().parse::<i64>(){
                    Ok(weight) => {
                        weight_int = weight;
                    },
                    Err(_) => {
                        println!("That is not a number");
                        continue;
                    }
                }
                // whether description is added
                let add_description = question_interface::yes_or_no_question(
                    false,
                    String::from("Do you want to add a description for this project?"),
                );
                let new_project: projects::Project;
                if add_description {
                    println!("Enter the description of the project");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).expect("Failed to read description");
                    new_project = projects::create_project_with_description(
                        project_name,
                        weight_int,
                        description
                    );
                }else{
                    new_project = projects::create_project(
                        project_name,
                        weight_int
                    );
                }
                projects = projects_manager.read_file_to_projects();
                projects.new_project(new_project);
                projects_manager.write_string_to_file(projects.to_string());
            },
            "q" => break,
            _ => {
                println!("Operation not supported");
            }
        }
    }
}

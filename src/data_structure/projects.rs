// projects module
pub mod projects {
    use crate::utils::rand_tools::random_tools;
    use crate::utils::date::date;
    use ansi_term::Colour;
    use serde::{Serialize, Deserialize};
    use serde_json;

    #[derive(Clone, Serialize, Deserialize)]
    // Project stores the data structure describing a project
    pub struct Project {
        project_name: String,
        weight: i64,
        description: String,
    }

    // create_project creates a project
    pub fn create_project(project_name: String, weight: i64) -> Project {
        Project {
            project_name: project_name,
            weight: weight,
            description: String::from("N/A"),
        }
    }

    // create_project_with_description creates a project with description
    pub fn create_project_with_description(
        project_name: String,
        weight: i64,
        description: String,
    ) -> Project {
        Project {
            project_name: project_name,
            weight: weight,
            description: description,
        }.clone()
    }

    impl Project {
        // get_project_name gets the name of this project
        pub fn get_project_name(&self) -> String {
            self.project_name.clone()
        }

        // get_project_description gets the description of the project
        pub fn get_project_description(&self) -> String {
            self.description.clone()
        }

        // get_project_weight gets the weight of the project
        pub fn get_project_weight(&self) -> i64 {
            self.weight
        }

        // show_project_info shows the info of the project
        pub fn show_project_info(&self) {
            println!();
            println!(
                "{}",
                Colour::Red.bold().paint(
                    format!(
                        "Project name: {}", self.get_project_name()
                    )
                )
            );
            println!(
                "{}",
                Colour::Green.italic().paint(
                    format!(
                        "Description: {}", self.get_project_description()
                    )
                )
            );
            println!(
                "{}",
                Colour::Blue.italic().paint(
                    format!(
                        "Weight: {}", self.get_project_weight()
                    )
                )
            );
            println!();
        }
    }

    // Projects stores the data structure describing list of projects
    #[derive(Clone, Serialize, Deserialize)]
    pub struct Projects {
        list_of_projects: Vec<Project>,
        table: random_tools::WAM,
        user_name: String,
    }

    // NewProjects creates a project list
    pub fn new_projects(user_name: String) -> Projects {
        Projects {
            list_of_projects: Vec::new(),
            table: random_tools::new_wam(Vec::new()),
            user_name: user_name,
        }.clone()
    }

    impl Projects {
        // get_random_weight gets the random weight of each project
        pub fn get_random_weight(&mut self) -> Vec<f64> {
            self.manage_table();
            let mut all_weights: i64 = 0;
            for n in &self.list_of_projects {
                all_weights += n.weight
            }
            let f_all_weights: f64 = all_weights as f64;
            let sub_weights: f64 = 1.0 / f_all_weights;
            let mut result: Vec<f64> = Vec::new();
            for n in &self.list_of_projects {
                let n_weight: f64 = n.weight as f64;
                result.push(n_weight * sub_weights)
            }
            result
        }

        // manage_table removes all null result
        fn manage_table(&mut self){
            for i in 0..self.list_of_projects.len(){
                let processed_i = self.list_of_projects.len() - 1 - i;
                if let Some(value) = self.list_of_projects.get(processed_i){

                }else{
                    self.list_of_projects.remove(processed_i);
                }
            }
        }

        // update_table updates the WAM table
        fn update_table(&mut self) {
            self.manage_table();
            let weight: Vec<f64> = self.get_random_weight();
            self.table = random_tools::new_wam(weight);
            self.table.construct_table();
        }

        // new_project adds new project to the Projects
        // Update table
        pub fn new_project(&mut self, project: Project) {
            self.list_of_projects.push(project);
            self.update_table();
        }

        // delete_project_by_name deletes certain project by name
        // Update table
        pub fn delete_project_by_name(&mut self, project_name: String) {
            for (idx, project) in self.list_of_projects.iter().enumerate() {
                if project.project_name == project_name {
                    self.list_of_projects.remove(idx);
                    break;
                }
            }
            self.update_table()
        }

        // get_all_projects get the list of projects
        pub fn get_all_projects(&mut self) -> Vec<Project> {
            self.manage_table();
            self.list_of_projects.clone()
        }

        // get_random_result get a random project from the projects
        pub fn get_random_result(&mut self) -> Option<Project> {
            self.manage_table();
            let chosen_idx: usize = self.table.sample() as usize;
            if chosen_idx >= self.list_of_projects.len(){
                None
            }else{
                Some(self.list_of_projects[chosen_idx].clone())
            }
        }

        // to_string turns this struct to the json format string
        pub fn to_string(&mut self) -> String {
            self.manage_table();
            match serde_json::to_string(&self){
                Ok(result) => result,
                Err(error) => {
                    panic!("{}",error);
                }
            }
        }

        // get_use_name gets the user name
        pub fn get_user_name(&self) -> String {
            self.user_name.clone()
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    // ProjectToday defines the random project selected for today
    pub struct ProjectToday{
        pub date: String,
        project: Project,
    }

    // new_project_today creates a new project
    pub fn new_project_today(project: Projects) -> ProjectToday {
        let mut new_list = project.clone();
        if let Some(selected_project) = new_list.get_random_result() {
            ProjectToday{
                date: date::get_date(),
                project:selected_project,
            }
        }else{
            panic!("Error getting random result");
        }
    }

    impl ProjectToday {
        // is_valid see if this project today is valid
        pub fn is_valid(&self) -> bool {
            if self.date == date::get_date() {
                true
            }else {
                false
            }
        }

        // to_string converts it to json-formatted string
        pub fn to_string(&self) -> String {
            match serde_json::to_string(&self){
                Ok(result) => result,
                Err(error) => {
                    panic!("{}",error);
                }
            }
        }

        // get_project gets the project
        pub fn get_project(&self) -> Project {
            self.project.clone()
        }
    }
}

#[cfg(test)]
// Test the projects module
mod test {
    use super::*;

    #[test]
    // Test the generation of weight
    fn test_weight_generation() {
        let project1: projects::Project = projects::create_project(String::from("1"), 2);
        let project2: projects::Project =
            projects::create_project_with_description(String::from("1"), 1, String::from("aaa"));
        let mut project_list: projects::Projects = projects::new_projects("abc".to_string());
        project_list.new_project(project1);
        project_list.new_project(project2);
        let result: Vec<f64> = project_list.get_random_weight();
        if result[0] != 2.0 * result[1] {
            panic!("The calculation of the weights failed!")
        }
        let mut sum: f64 = 0.0;
        for num in result {
            sum += num;
        }
        if sum != 1.0 {
            panic!(
                "The calculation of the weights failed due to the total weight {} not equals to 1.0!",
                sum,
            )
        }
    }

    #[test]
    // Test the deletion of project
    fn test_delete_project_by_name() {
        let project1: projects::Project = projects::create_project(String::from("1"), 2);
        let project2: projects::Project =
            projects::create_project_with_description(String::from("abc"), 3, String::from("aaa"));
        let mut project_list: projects::Projects = projects::new_projects("abc".to_string());
        project_list.new_project(project1);
        project_list.new_project(project2);
        project_list.delete_project_by_name(String::from("abc"));
        for project in project_list.get_all_projects() {
            if project.get_project_name() == String::from("abc") {
                panic!("The project didn't successfully deleted");
            }
        }
        if project_list.get_all_projects().len() != 1 {
            panic!("Wrong project may be deleted");
        }
    }

    #[test]
    // Test get_random_result
    fn test_random_result() {
        let project1: projects::Project = projects::create_project(String::from("1"), 2);
        let project2: projects::Project =
            projects::create_project_with_description(String::from("abc"), 3, String::from("aaa"));
        let mut project_list: projects::Projects = projects::new_projects("abc".to_string());
        project_list.new_project(project1);
        project_list.new_project(project2);
        let mut passed = false;
        if let Some(result) = project_list.get_random_result() {
            let list = project_list.get_all_projects();
            for i in 0..list.len() {
                if let Some(value) = list.get(i) {
                    if value.get_project_name() == result.get_project_name() {
                        passed = true;
                        break
                    }
                }else{
                    panic!("There is a null inside the project list")
                }
            }
            if !passed {
                panic!("The selected project does not exist!");
            }
        } else {
            panic!("The calculation of the results failed!");
        }
    }

    #[test]
    // Test to_string
    fn test_to_string() {
        let project1: projects::Project = projects::create_project(String::from("1"), 2);
        let project2: projects::Project =
            projects::create_project_with_description(String::from("abc"), 3, String::from("aaa"));
        let mut project_list: projects::Projects = projects::new_projects("abc".to_string());
        project_list.new_project(project1);
        project_list.new_project(project2);
        project_list.to_string();
    }

    #[test]
    // Test project for today
    fn test_project_today() {
        let project1: projects::Project = projects::create_project(String::from("1"), 2);
        let project2: projects::Project =
            projects::create_project_with_description(String::from("abc"), 3, String::from("aaa"));
        let mut project_list: projects::Projects = projects::new_projects("abc".to_string());
        project_list.new_project(project1);
        project_list.new_project(project2);
        let project_today: projects::ProjectToday = projects::new_project_today(project_list);
        assert_eq!(project_today.is_valid(), true);
        project_today.to_string();
    }
}

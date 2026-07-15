// projects module
mod projects {
    use crate::utils::rand_tools::random_tools;

    #[derive(Clone)]
    // Project stores the data structure describing a project
    pub struct Project {
        project_name: String,
        weight: i64,
        description: String,
    }

    // create_project creates a project
    pub fn create_project(project_name: String, weight: i64) -> Project {
        return Project {
            project_name: project_name,
            weight: weight,
            description: String::from("N/A"),
        };
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
    }

    // Projects stores the data structure describing list of projects
    #[derive(Clone)]
    pub struct Projects {
        list_of_projects: Vec<Project>,
        table: random_tools::WAM,
    }

    // NewProjects creates a project list
    pub fn new_projects() -> Projects {
        Projects {
            list_of_projects: Vec::new(),
            table: random_tools::new_wam(Vec::new()),
        }.clone()
    }

    impl Projects {
        // get_random_weight gets the random weight of each project
        pub fn get_random_weight(&self) -> Vec<f64> {
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

        // update_table updates the WAM table
        fn update_table(&mut self) {
            let weight: Vec<f64> = self.get_random_weight();
            self.table = random_tools::new_wam(weight);
            self.table.construct_table();
        }

        // new_project adds new project to the Projects
        // Update table
        pub fn new_project(&mut self, project: Project) {
            &self.list_of_projects.push(project);
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
        }

        // get_all_projects get the list of projects
        pub fn get_all_projects(&self) -> Vec<Project> {
            self.list_of_projects.clone()
        }
        // get_random_result get a random project from the projects
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
        let mut project_list: projects::Projects = projects::new_projects();
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
        let mut project_list: projects::Projects = projects::new_projects();
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
}

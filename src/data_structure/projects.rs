// Project stores the data structure describing a project
struct Project {
    project_name: String,
    weight: i64,
}

// Projets stores the data strucutre describing list of projects
struct Projects {
    list_of_projects: Vec<Project>
}

impl Projects {
    // get_random_weight gets the random weight of each project
    fn get_random_weight(&self) -> Vec<f64>{
        let mut all_weights: i64 = 0;
        for n in &self.list_of_projects{
            all_weights += n.weight
        }
        let f_all_weights: f64 = all_weights as f64;
        let sub_weights: f64 = 1.0/f_all_weights;
        let mut result: Vec<f64> = Vec::new();
        for n in &self.list_of_projects{
            let n_weight: f64 = n.weight as f64;
            result.push(n_weight*sub_weights)
        }
        return result
    }
}

pub mod data_manager {
    use crate::data_manager::file_manager::file_manager;
    use crate::data_structure::projects::projects;

    // DataManager defines the manager of the data of the projects
    pub struct DataManager {
        file_name: String,
        project_list: projects::Projects,
    }
}
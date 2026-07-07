pub mod projects;

// Project stores the data structure describing a project
struct Project {
    project_name: String,
    weight: Int,
}

// Projets stores the data strucutre describing list of projects
struct Projects {
    list_of_projects: Vec<String>
}

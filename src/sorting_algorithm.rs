use crate::file_utils::read_xsd_file;

use std::fs;
use std::collections::{HashMap, HashSet};
use quick_xml::Reader;
use quick_xml::name::QName;
use quick_xml::events::Event::{Start, Empty, Eof};

// Create a map of file dependencies
pub fn create_file_dependencies(
    folder_path: &str, 
    file_dependencies: &mut HashMap<String, HashSet<String>>
) {

    // Iterate over the files in the folder
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Convert the path to a string and replace backslashes with forward slashes
        let mut path_location = path.to_str().unwrap().to_string().replace("\\", "/");
        if path_location.ends_with(".xsd") || path_location.ends_with(".dtd") {
            let file_name = path_location.split("/").collect::<Vec<&str>>().last().unwrap().to_string();
            path_location = path_location.replace(file_name.as_str(), "");
        }

        // If the entry is a directory, call this function recursively
        if path.is_dir() {
            create_file_dependencies(path.to_str().unwrap(), file_dependencies);
        }

        // Check if the entry is a file and has a `.xsd` extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("xsd") {
            let current_file = path.to_str().unwrap().replace("\\", "/");
            let content = read_xsd_file(path.to_str().unwrap()).unwrap();
            let mut reader = Reader::from_str(&content);
            let mut dependencies = HashSet::new();

            // Retrieve the dependencies of the current file
            loop {
                match reader.read_event() {

                    Ok(Start(ref e)) | Ok(Empty(ref e)) => {
                        if e.name() == QName(b"xs:import") {
                            if let Some(location) = e.attributes().filter_map(|a| a.ok())
                                .find(|a| a.key == QName(b"schemaLocation"))
                                .and_then(|a| String::from_utf8(a.value.to_vec()).ok()) {

                                // Add the dependency to the set
                                let location = path_location.clone() + &location;
                                dependencies.insert(location);
                            }
                        }
                    },
                    Ok(Eof) => break,
                    _ => (),
                }
            }

            file_dependencies.insert(current_file.to_string(), dependencies);
        }
    }
}

// Sort files based on dependencies
pub fn sort_files(file_dependencies: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut sorted_files = Vec::new();
    let mut visited = HashSet::new();
    let mut temp_stack = HashSet::new();

    for file_name in file_dependencies.keys() {
        topological_sort(file_name, file_dependencies, &mut sorted_files, &mut visited, &mut temp_stack);
    }

    sorted_files
}

// Depth-first traversal and sorting
// Changed three .xsd files because they had circular dependencies:
// WorkingSiteTrade_CallForOfferWorkingSite.xsd, WorkingSiteTrade_WorkingSite.xsd and WorkingSiteTrade_Assortment.xsd
fn topological_sort(
    file_name: &str,
    file_dependencies: &HashMap<String, HashSet<String>>,
    sorted_files: &mut Vec<String>,
    visited: &mut HashSet<String>,
    temp_stack: &mut HashSet<String>,
) {

    if temp_stack.contains(file_name) {
        panic!("Cycle detected! Files cannot be processed due to circular imports: {}", file_name);
    }

    if !visited.contains(file_name) {
        temp_stack.insert(file_name.to_string());

        if let Some(dependencies) = file_dependencies.get(file_name) {
            for dep in dependencies {
                topological_sort(dep, file_dependencies, sorted_files, visited, temp_stack);
            }
        }

        temp_stack.remove(file_name);
        visited.insert(file_name.to_string());
        sorted_files.push(file_name.to_string());
    }
}

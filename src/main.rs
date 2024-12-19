use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::file_utils::{element_definitions_to_file, read_xsd_file, structs_to_file};

use std::collections::{HashMap, HashSet};
use std::fs;
use quick_xml::Reader;
use quick_xml::events::Event::{Start, Empty, Eof};
use quick_xml::name::QName;

fn main() {
    let folder_path = "./MetsatietostandardiskeematV33.03"; // Path to your folder containing XSD files
    
    let mut file_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    create_file_dependencies(folder_path, &mut file_dependencies);

    let sorted_files = sort_files(&file_dependencies);

    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let mut total_struct_count = 0;
    let mut total_element_count = 0;

    for file in sorted_files {
        //println!("Processing file: {}", file);
        let counts = process_xsd_file(&file, &mut structs, &mut element_definitions);
        total_struct_count += counts.0;
        total_element_count += counts.1;
    }
    
    structs_to_file(&structs, "structs/__all_structs.rs").unwrap();
    element_definitions_to_file(&element_definitions, "structs/__all_element_definitions.rs").unwrap();


    println!("Total number of structs: {}", total_struct_count);
    println!("Actual number of structs: {}", structs.len());
    println!("Total number of element definitions: {}", total_element_count);
    println!("Actual number of element definitions: {}", element_definitions.len());
/*     let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let file_name = "./structs.rs";
    let elements_file_name = "./element_definitions.rs";

    let content = read_xsd_file("schema.xsd").unwrap();
    let mut reader = Reader::from_str(&content);
    create_structs(&mut reader, &mut structs, &mut element_definitions, &content);

    structs_to_file(&structs, &file_name).unwrap();
    element_definitions_to_file(&element_definitions, &elements_file_name).unwrap(); */
}

fn create_file_dependencies(folder_path: &str, file_dependencies: &mut HashMap<String, HashSet<String>>) {

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
fn sort_files(file_dependencies: &HashMap<String, HashSet<String>>) -> Vec<String> {
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

// Creates the structs and element definitions and writes them to files in the `structs` folder
fn process_xsd_file(current_file: &str, structs: &mut HashMap<String, XMLStruct>, element_definitions: &mut HashMap<String, String>) -> (usize, usize) {
    let mut new_structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut new_element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let mut file_name = current_file.split("/").collect::<Vec<&str>>().last().unwrap().to_string();
    file_name = "./structs/".to_string() + &file_name.replace(".xsd", ".rs");
    
    let elements_file_name = "./structs/_ed_".to_string() + &file_name.replace("./structs/", "");

    let content = read_xsd_file(current_file).unwrap();
    let mut reader = Reader::from_str(&content);

    let current_definitions = element_definitions.clone();

    create_structs(&mut reader, &mut new_structs, element_definitions, &content);

    for (key, value) in element_definitions.iter() {
        if !current_definitions.contains_key(key) {
            new_element_definitions.insert(key.to_string(), value.to_string());
        }
    }

    for (key, value) in new_structs.iter() {
        if structs.contains_key(key) {
            
            for field in value.fields.iter() {
                if !structs.get(key).unwrap().fields.iter().any(|f| f.name == field.name) {
                    structs.get_mut(key).unwrap().fields.push(field.clone());
                    //println!("Field {}: {} added to struct: {}", field.name, field.field_type, key);
                }
            }
        }
    }

    // Write the new structs to a file
    if new_structs.len() > 0 {
        structs_to_file(&new_structs, &file_name).unwrap();
    }

    structs.extend(new_structs.clone());

    // Write the new element definitions to a file
    if new_element_definitions.len() > 0 {
        element_definitions_to_file(&new_element_definitions, &elements_file_name).unwrap();
    }

    (new_structs.len(), new_element_definitions.len())
}
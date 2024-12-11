use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::file_utils::{element_definitions_to_file, read_xsd_file, structs_to_file};

use std::collections::{HashMap, HashSet};
use std::fs;
use quick_xml::Reader;
use quick_xml::events::Event::{Start, Empty, Eof};
use quick_xml::name::QName;

fn main() {
    let folder_path = "./MetsatietostandardiskeematV33.03/V33"; // Path to your folder containing XSD files
    
    let mut file_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    create_file_dependencies(folder_path, &mut file_dependencies);

    let sorted_files = sort_files(&file_dependencies);
    for file in sorted_files {
        println!("{}", file);
        println!("  {:#?}", file_dependencies.get(&file));
        println!();
    }
/*     for entry in fs::read_dir(folder_path).unwrap() {
        let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
        let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the entry is a file and has a `.xsd` extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("xsd") {
            let current_file = path.file_name().unwrap().to_str().unwrap();
            let mut file_name = current_file.replace(".xsd", ".rs");
            file_name = "./structs/".to_string() + &file_name;

            let elements_file_name = "./structs/".to_string() + &current_file.replace(".xsd", "_ed.rs");
            
            let content = read_xsd_file(path.to_str().unwrap()).unwrap();
            let mut reader = Reader::from_str(&content);
            create_structs(&mut reader, &mut structs, &mut element_definitions);

            structs_to_file(&structs, &file_name).unwrap();
            element_definitions_to_file(&element_definitions, &elements_file_name).unwrap();
        }
    } */

/*     let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let file_name = "./structs.rs";
    let elements_file_name = "./element_definitions.rs";

    let content = read_xsd_file("schema.xsd").unwrap();
    let mut reader = Reader::from_str(&content);
    create_structs(&mut reader, &mut structs, &mut element_definitions);

    structs_to_file(&structs, &file_name).unwrap();
    element_definitions_to_file(&element_definitions, &elements_file_name).unwrap(); */
}

fn create_file_dependencies(folder_path: &str, file_dependencies: &mut HashMap<String, HashSet<String>>) {

    // Iterate over the files in the folder
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the entry is a file and has a `.xsd` extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("xsd") {
            let current_file = path.file_name().unwrap().to_str().unwrap();
            let content = read_xsd_file(path.to_str().unwrap()).unwrap();
            let mut reader = Reader::from_str(&content);
            let mut dependencies = HashSet::new();

            // Retrieve the dependencies of the current file
            loop {
                match reader.read_event() {

                    Ok(Start(ref e)) => {
                        if e.name() == QName(b"xs:import") {
                            if let Some(location) = e.attributes().filter_map(|a| a.ok())
                                .find(|a| a.key == QName(b"schemaLocation"))
                                .and_then(|a| String::from_utf8(a.value.to_vec()).ok()) {
                                dependencies.insert(location);
                            }
                        }
                    },
                    Ok(Empty(ref e)) => {
                        if e.name() == QName(b"xs:import") {
                            if let Some(location) = e.attributes().filter_map(|a| a.ok())
                                .find(|a| a.key == QName(b"schemaLocation"))
                                .and_then(|a| String::from_utf8(a.value.to_vec()).ok()) {
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
// Changed two .xsd files because they had circular dependencies: WorkingSiteTrade_Product.xsd and WorkingSiteTrade_Common.xsd
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
use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::file_utils::{read_xsd_file, structs_and_definitions_to_file};
use struct_generator::sorting_algorithm::{create_file_dependencies, sort_files};
use struct_generator::string_utils::{capitalize_first, remove_colon_from_string};

use std::collections::{HashMap, HashSet};
use quick_xml::Reader;

fn main() {
    let folder_path = "./MetsatietostandardiskeematV33.03"; // Path to your folder containing XSD files
    
    let mut file_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    create_file_dependencies(folder_path, &mut file_dependencies);

    let sorted_files = sort_files(&file_dependencies);

    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let mut total_struct_count = 0;
    let mut total_element_count = 0;

    let prefixes = &mut HashMap::new();

    for file in sorted_files {
        //println!("Processing file: {}", file);
        let counts = process_xsd_file(&file, &mut structs, &mut element_definitions, prefixes);
        total_struct_count += counts.0;
        total_element_count += counts.1;
    }
    
    prefixes_to_struct_keys(&mut structs, prefixes);

    fix_fields_with_colons(&mut structs);

    change_circular_field_types(&mut structs);

    //structs_to_file(&structs, "structs/__all_structs.rs").unwrap();
    //element_definitions_to_file(&element_definitions, "structs/__all_element_definitions.rs", prefixes).unwrap();
    structs_and_definitions_to_file(&structs, &element_definitions, prefixes, "src/__structs_and_definitions.rs").unwrap();

    println!("Total number of structs: {}", total_struct_count);
    println!("Actual number of structs: {}", structs.len());
    println!("Total number of element definitions: {}", total_element_count);
    println!("Actual number of element definitions: {}", element_definitions.len());

    println!("Prefix count: {}", prefixes.len());
    
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

// Modify the keys of the structs to include the prefixes
fn prefixes_to_struct_keys(
    structs: &mut HashMap<String, XMLStruct>, 
    prefixes: &mut HashMap<String, String>
) {
    let mut new_structs = HashMap::new();

    for prefix in prefixes.iter() {
        structs.iter().for_each(|(key, value)| {
            let p_value = capitalize_first(prefix.1);

            if prefix.0.replace(&p_value, "") == *key {
                let new_key = format!("{}{}", p_value, key);
                new_structs.insert(new_key.clone(), value.clone());
                //println!("{} -> {}", key, new_key);
            } else {
                new_structs.insert(key.clone(), value.clone());
            }
        });
    }
    
    *structs = new_structs;
}

// Remove colons from field names and field types
fn fix_fields_with_colons(structs: &mut HashMap<String, XMLStruct>) {
    for (_, value) in structs.iter_mut() {
        for field in value.fields.iter_mut() {
            if field.name.contains(':') {
                field.name = capitalize_first(&remove_colon_from_string(&field.name));
            }

            if field.field_type.contains(":") {
                if field.field_type.contains("Option<Vec<") {
                    field.field_type = field.field_type.replace("Option<Vec<", "");
                    field.field_type = field.field_type.replace(">>", "");

                    field.field_type = capitalize_first(&remove_colon_from_string(&field.field_type));
                    field.field_type = format!("Option<Vec<{}>>", field.field_type);

                } else if field.field_type.contains("Vec<") {
                    field.field_type = field.field_type.split('<').collect::<Vec<&str>>()[1].to_string();
                    field.field_type = field.field_type.replace(">", "");

                    field.field_type = capitalize_first(&remove_colon_from_string(&field.field_type));
                    field.field_type = format!("Vec<{}>", field.field_type);

                } else if field.field_type.contains("Option<") {
                    field.field_type = field.field_type.split('<').collect::<Vec<&str>>()[1].to_string();
                    field.field_type = field.field_type.replace(">", "");

                    field.field_type = capitalize_first(&remove_colon_from_string(&field.field_type));
                    field.field_type = format!("Option<{}>", field.field_type);
                }
            }
        }
    }
}

// Creates the structs and element definitions and writes them to files in the `structs` folder
fn process_xsd_file(
    current_file: &str, 
    structs: &mut HashMap<String, XMLStruct>, 
    element_definitions: &mut HashMap<String, String>, 
    prefixes: &mut HashMap<String, String>
) -> (usize, usize) {

    let mut new_structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut new_element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    let mut file_name = current_file.split("/").collect::<Vec<&str>>().last().unwrap().to_string();
    file_name = "./structs/".to_string() + &file_name.replace(".xsd", ".rs");

    let content = read_xsd_file(current_file).unwrap();
    let mut reader = Reader::from_str(&content);

    let current_definitions = element_definitions.clone();

    create_structs(&mut reader, &mut new_structs, element_definitions, &content, prefixes);

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
    if new_structs.len() > 0 || new_element_definitions.len() > 0 {
        structs_and_definitions_to_file(&new_structs, &new_element_definitions, prefixes, &file_name).unwrap();
    }

    structs.extend(new_structs.clone());

    (new_structs.len(), new_element_definitions.len())
}

// If a struct has a field that is a reference to itself, change the field type to something else
fn change_circular_field_types(structs: &mut HashMap<String, XMLStruct>) {
    for (key, value) in structs.iter_mut() {

        // Skip structs that start with "Xs"
        if key.starts_with("Xs") {
            continue;
        }

        for field in value.fields.iter_mut() {
            if field.field_type == *key {
                if key.ends_with("PlannedResourceType") {
                    field.field_type = "CiContactInformationType".to_string();
                } else if key.ends_with("AreaDecreaseType") {
                    field.field_type = "f64".to_string();
                } else if key.contains("Date") {
                    field.field_type = "chrono::NaiveDate".to_string();
                } else {
                    field.field_type = "String".to_string();
                }
            }
        }
    }
}
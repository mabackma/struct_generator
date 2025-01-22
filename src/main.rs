use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::file_utils::{read_xsd_file, structs_and_definitions_to_file, RUST_TYPES};
use struct_generator::sorting_algorithm::{create_file_dependencies, sort_files};
use struct_generator::string_utils::{capitalize_first, handle_prefix, remove_colon_from_string, remove_prefix, XSD_TO_RUST};

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

    missing_definitions(&mut element_definitions, &structs, prefixes);

    remove_duplicates_from_element_definitions(&mut element_definitions, &structs);

    replace_field_types_from_definitions(&element_definitions, &mut structs);

    remove_prefixes_from_missing_types(&mut element_definitions, &structs, prefixes);

    fix_lowercase_types(&mut element_definitions, prefixes);

    //structs_to_file(&structs, "structs/__all_structs.rs").unwrap();
    //element_definitions_to_file(&element_definitions, "structs/__all_element_definitions.rs", prefixes).unwrap();
    structs_and_definitions_to_file(&structs, &element_definitions, prefixes, "src/__structs_and_definitions.rs").unwrap();

    println!("Total number of structs: {}", total_struct_count);
    println!("Actual number of structs: {}", structs.len());
    println!("Total number of element definitions: {}", total_element_count);
    println!("Actual number of element definitions: {}", element_definitions.len());

    println!("Prefix count: {}\n", prefixes.len());

    print_missing_fields(&structs, &element_definitions, prefixes);

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

// Creates the structs and element definitions and writes them to files in the `structs` folder
// Returns the number of structs and element definitions created
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

// Modify the keys of the structs to include the prefixes
fn prefixes_to_struct_keys(
    structs: &mut HashMap<String, XMLStruct>, 
    prefixes: &mut HashMap<String, String>
) {
    let mut new_structs = HashMap::new();

    for prefix in prefixes.iter() {
        structs.iter().for_each(|(key, value)| {

            // Capitalized prefix (e.g. "Co", "Gml")
            let p_value = capitalize_first(prefix.1);

            if prefix.0.replace(&p_value, "") == *key {
                let new_key = format!("{}{}", p_value, key);
                new_structs.insert(new_key.clone(), value.clone());
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

// If a struct has a field that is a reference to itself, change the field type to something else
fn change_circular_field_types(structs: &mut HashMap<String, XMLStruct>) {
    for (key, value) in structs.iter_mut() {

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

// Generate missing definitions in element_definitions
fn missing_definitions(
    element_definitions: &mut HashMap<String, String>,
    structs: &HashMap<String, XMLStruct>,
    prefixes: &HashMap<String, String>
) {

    let mut new_definitions = element_definitions.clone();

    for s in structs.iter() {
        for f in s.1.fields.iter() {
            let f_type = f.field_type.replace("Option<", "").replace("Vec<", "").replace(">", "").replace(">", "");

            if !structs.contains_key(&f_type) && !new_definitions.contains_key(&f_type) {

                if !new_definitions.contains_key(&f_type) && !RUST_TYPES.contains(&f_type.as_str()) {
                  
                    if prefixes.contains_key(&f_type) {
                        let p = prefixes.get(&f_type).unwrap();
                        let skip = p.len();
                        let new_type = f_type[skip..].to_string();
                        
                        new_definitions.insert(f_type.clone(), new_type);
                    }
                    
                }
            }
        }
    }

    *element_definitions = new_definitions;
}

// Remove duplicate structs from element_definitions
fn remove_duplicates_from_element_definitions(
    element_definitions: &mut HashMap<String, String>, 
    structs: &HashMap<String, XMLStruct>
) {

    let mut new_definitions = element_definitions.clone();

    for s_key in structs.keys() {
        for el_key in element_definitions.keys() {
            if s_key == el_key {
                new_definitions.remove(el_key);
            }
        }
    }

    *element_definitions = new_definitions;
}

// If a type is missing, it might be because it has a prefix that is not in the struct keys
fn remove_prefixes_from_missing_types(
    element_definitions: &mut HashMap<String, String>, 
    structs: &HashMap<String, XMLStruct>, 
    prefixes: &mut HashMap<String, String>
) {

    let mut new_definitions = element_definitions.clone();

    for (_, typ) in new_definitions.iter_mut() {
        let el_type = handle_prefix(typ, prefixes);

        if !structs.contains_key(&el_type) && !element_definitions.contains_key(&el_type) {

            if !el_type.starts_with("Xs") && !el_type.starts_with("Xlink") && !el_type.starts_with("xlink") && !XSD_TO_RUST.contains_key(&el_type.as_str()) {
                let new_type = remove_prefix(&el_type, prefixes);
                *typ = new_type;
            }
        }
    }

    *element_definitions = new_definitions;
}


fn replace_field_types_from_definitions(
    element_definitions: &HashMap<String, String>,
    structs: &mut HashMap<String, XMLStruct>
) {
    let mut new_structs = structs.clone();

    for (_, s) in new_structs.iter_mut() {
        for f in s.fields.iter_mut() {
            if f.field_type.contains("Option<Vec<") {
                let field_type = f.field_type.replace("Option<", "").replace("Vec<", "").replace(">", "");

                if element_definitions.contains_key(&field_type) {                    
                    f.field_type = f.field_type.replace(&field_type, element_definitions.get(&field_type).unwrap());
                }
            }
        }
    }

    *structs = new_structs;
}

fn fix_lowercase_types(
    element_definitions: &mut HashMap<String, String>,
    prefixes: &mut HashMap<String, String>
) {
    let mut new_definitions = element_definitions.clone();

    for (el_key, typ) in element_definitions.iter() {
        let mut el_type = handle_prefix(typ, prefixes);

        if el_type.chars().next().map_or(false, char::is_lowercase) && !RUST_TYPES.contains(&el_type.as_str()){

            if let Some(rust_type) = XSD_TO_RUST.get(&el_type.as_str()) {
                el_type = rust_type.to_string();
            } else {
                el_type = capitalize_first(&el_type);
            }

            new_definitions.remove(el_key);
            new_definitions.insert(el_key.to_string(), el_type);
        }
    }

    *element_definitions = new_definitions;
}

fn print_missing_fields(
    structs: &HashMap<String, XMLStruct>,
    element_definitions: &HashMap<String, String>,
    prefixes: &mut HashMap<String, String>
) {
    for s in structs.iter() {
        for f in s.1.fields.iter() {
            let f_type = f.field_type.replace("Option<", "").replace("Vec<", "").replace(">", "").replace(">", "");

            if !structs.contains_key(&f_type) && !element_definitions.contains_key(&f_type) {
                
                if !RUST_TYPES.contains(&f_type.as_str()) {
                    println!("STRUCT {} HAS MISSING TYPE FOR FIELD: {} -> {}", s.0, f.name, f_type);
                }
            }
        }
    }

    let gis_types = vec!["Point", "Polygon", "LineString", "MultiPolygon", "MultiLineString"];

    for (el_key, typ) in element_definitions.iter() {
        let el_type = handle_prefix(typ, prefixes);

        if !structs.contains_key(&el_type) && !element_definitions.contains_key(&el_type) {

            if !XSD_TO_RUST.contains_key(&el_type.as_str()) && !gis_types.contains(&el_type.as_str()) && !RUST_TYPES.contains(&typ.as_str()) {
                println!("  {}: {}", el_key, el_type);
            }
        }
    }
}
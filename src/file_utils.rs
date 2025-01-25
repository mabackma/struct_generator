use crate::create_structs::XMLStruct;
use crate::string_utils::{handle_prefix, remove_prefix, to_snake_case, XSD_TO_RUST};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::fs;

pub const RUST_TYPES: &[&str] = &[
    "bool", "f64", "f32", "i32", "u32", "i8", "i16", "i64", 
    "u8", "u16", "u64", "String", "NaiveDate", 
    "NaiveTime", "NaiveDateTime", 
    "std::time::Duration", "Vec<u8>",
];

// Reads an XML file and returns its contents as a string
pub fn read_xsd_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut xsd_string = String::new();
    file.read_to_string(&mut xsd_string)?;

    // Remove Byte Order Mark (BOM) if it exists
    if xsd_string.starts_with("\u{feff}") {
        xsd_string = xsd_string.trim_start_matches("\u{feff}").to_string();
    }

    Ok(xsd_string)
}

// Save the structs and definitions to a file
pub fn structs_and_definitions_to_file(
    structs: &HashMap<String, XMLStruct>, 
    element_definitions: &HashMap<String, String>, 
    prefixes: &mut HashMap<String, String>, 
    file_name: &str
) -> io::Result<()> {

    let mut structs_string = String::new();

    structs_string.push_str("use serde::{Serialize, Deserialize};\n");
    structs_string.push_str("use chrono::{NaiveDate, NaiveTime, NaiveDateTime};\n\n");
    structs_string.push_str("use geo::{Point, Polygon, MultiPolygon, LineString};\n\n");


    // Add element definitions to the string
    let definitions_string = generate_element_definitions(element_definitions, prefixes);
    structs_string.push_str(&definitions_string);

    // Add structs to the string
    let struct_string = generate_structs(structs, prefixes);
    structs_string.push_str(&struct_string);

    create_directory(&file_name);

    // Write the string to the file
    let mut file = File::create(file_name)?;
    file.write_all(structs_string.as_bytes())?;

    Ok(())
}

// Build the element definitions as a string
fn generate_element_definitions(
    element_definitions: &HashMap<String, String>, 
    prefixes: &mut HashMap<String, String>
) -> String {
    let mut definitions_string = String::new();

    for (name, typ) in element_definitions.iter() {
        definitions_string.push_str("#[derive(Serialize, Deserialize, Debug)]\n");
        definitions_string.push_str(&format!("pub struct {} {{\n", name));
        definitions_string.push_str("    #[serde(flatten)]\n");

        let field_type = handle_prefix(typ, prefixes);
        
        definitions_string.push_str(&format!("    pub {}: {},\n", to_snake_case(name), field_type));

        definitions_string.push_str("}\n\n");
    }

    definitions_string
}

// Build the structs as a string
fn generate_structs(
    structs: &HashMap<String, XMLStruct>,
    prefixes: &mut HashMap<String, String>
) -> String {

    let mut structs_string = String::new();

    for (name, xml_struct) in structs.iter() {
        structs_string.push_str(&format!("#[derive(Debug, Serialize, Deserialize)]"));
        structs_string.push_str(&format!("\npub struct {} {{\n", name));

        for field in xml_struct.fields.iter() {
            let mut field_type = field.field_type.to_string();
            
            field_type = if let Some(ft) = XSD_TO_RUST.get(&field_type.replace("Xs", "")) {
                ft.to_string()
            } else {
                field_type.to_string()
            }; 

            if field.name == "base" {

                if !RUST_TYPES.contains(&field_type.as_str()) {
                    structs_string.push_str(&format!("    #[serde(flatten)]\n"));
                } 
                
            } else if field.field_type.starts_with("Option<") {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", remove_prefix(&field.name, prefixes)));
            } else {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\")]\n", remove_prefix(&field.name, prefixes)));
            }
            
            structs_string.push_str(&format!("    pub {}: {},\n", to_snake_case(&field.name), field_type));
        }

        structs_string.push_str("}\n\n");
    }

    structs_string
}

// Create a directory for the file if it does not exist
fn create_directory(file_name: &str) {
    let path_vec = file_name.split("/").collect::<Vec<&str>>();
    let path = path_vec[..path_vec.len() - 1].join("/");

    // Check if the directory exists
    if !Path::new(&path).exists() {
        // Create the directory since it does not exist
        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory created for file: {}", file_name),
            Err(e) => eprintln!("Failed to create directory for file: {}. Error: {}", file_name, e),
        }
    }
}
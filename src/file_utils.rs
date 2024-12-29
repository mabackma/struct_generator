use crate::create_structs::XMLStruct;
use crate::string_utils::{handle_prefix, to_snake_case};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use phf::{Map, phf_map};
use std::path::Path;
use std::fs;

static XSD_TO_RUST: Map<&'static str, &str> = phf_map! {
    "boolean" => "bool",
    "decimal" => "f64",
    "float" => "f32",
    "double" => "f64",
    "integer" => "i32",
    "positiveInteger" => "u32",
    "nonPositiveInteger" => "i32",
    "negativeInteger" => "i32",
    "nonNegativeInteger" => "u32",
    "byte" => "i8",
    "short" => "i16",
    "int" => "i32",
    "long" => "i64",
    "unsignedByte" => "u8",
    "unsignedShort" => "u16",
    "unsignedInt" => "u32",
    "unsignedLong" => "u64",
    "string" => "String",
    "normalizedString" => "String",
    "token" => "String",
    "language" => "String",
    "Name" => "String",
    "NCName" => "String",
    "ID" => "String",
    "IDREF" => "String",
    "ENTITY" => "String",
    "anyURI" => "String",
    "date" => "chrono::NaiveDate",
    "time" => "chrono::NaiveTime",
    "dateTime" => "chrono::NaiveDateTime",
    "duration" => "std::time::Duration",
    "gYear" => "chrono::NaiveDate",
    "gMonth" => "chrono::NaiveDate",
    "gDay" => "chrono::NaiveDate",
    "gYearMonth" => "chrono::NaiveDate",
    "gMonthDay" => "chrono::NaiveDate",
    "dateTimeStamp" => "chrono::NaiveDateTime",
    "base64Binary" => "Vec<u8>",
    "hexBinary" => "Vec<u8>",
    "anySimpleType" => "String",
    "CoDateYYYY-MMOrYYYY-MM-DDType" => "chrono::NaiveDate",
};

const RUST_TYPES: &[&str] = &[
    "bool", "f64", "f32", "i32", "u32", "i8", "i16", "i64", 
    "u8", "u16", "u64", "String", "chrono::NaiveDate", 
    "chrono::NaiveTime", "chrono::NaiveDateTime", 
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

    // Add element definitions to the string
    structs_string.push_str(&generate_element_definitions(element_definitions, prefixes));

    // Add structs to the string
    structs_string.push_str(&generate_structs(structs));

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
        definitions_string.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
        definitions_string.push_str(&format!("pub struct {} {{\n", name));
        definitions_string.push_str("    #[serde(flatten)]\n");

        let field_type = handle_prefix(typ, prefixes);

        let typ = if let Some(ft) = XSD_TO_RUST.get(&field_type.replace("Xs", "")) {
            ft.to_string()
        } else {
            field_type.to_string()
        };

        definitions_string.push_str(&format!("    pub {}: {},\n", to_snake_case(name), typ));
        definitions_string.push_str("}\n\n");
    }

    definitions_string
}

// Build the structs as a string
fn generate_structs(structs: &HashMap<String, XMLStruct>) -> String {
    let mut structs_string = String::new();

    for (name, xml_struct) in structs.iter() {
        structs_string.push_str(&format!("#[derive(Debug, Serialize, Deserialize)]"));
        structs_string.push_str(&format!("\npub struct {} {{\n", name));

        for field in xml_struct.fields.iter() {
            // Get Rust primitive type from XSD type
            let field_type = if let Some(field_type) = XSD_TO_RUST.get(&field.field_type.replace("Xs", "")) {
                field_type.to_string()
            } else {
                field.field_type.to_string()
            };

            if field.name == "base" && !RUST_TYPES.contains(&field_type.as_str()) {
                structs_string.push_str(&format!("    #[serde(flatten)]\n"));
            } else if field.field_type.starts_with("Option<") {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name));
            } else {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
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
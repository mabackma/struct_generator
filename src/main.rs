use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::string_utils::{remove_prefix, to_snake_case};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use quick_xml::Reader;

fn main() {
    let folder_path = "./MetsatietostandardiskeematV33.03/V33"; // Path to your folder containing XSD files
    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    // Read and process all XSD files in the folder
    let xsd_files = process_folder(folder_path).unwrap();

    for xsd_content in xsd_files.iter() {
        let mut reader = Reader::from_str(xsd_content);
        create_structs(&mut reader, &mut structs, &mut element_definitions);
    }

    structs_to_file(&structs, "structs.rs").unwrap();
    element_definitions_to_file(&element_definitions, "element_definitions.rs").unwrap();
}

fn process_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut xsd_contents = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has a `.xsd` extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("xsd") {
            let content = read_xsd_file(path.to_str().unwrap())?;
            xsd_contents.push(content);
        }
    }

    Ok(xsd_contents)
}

// Reads an XML file and returns its contents as a string
fn read_xsd_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut xsd_string = String::new();
    file.read_to_string(&mut xsd_string)?;

    // Remove Byte Order Mark (BOM) if it exists
    if xsd_string.starts_with("\u{feff}") {
        xsd_string = xsd_string.trim_start_matches("\u{feff}").to_string();
    }

    Ok(xsd_string)
}

fn structs_to_file(structs: &HashMap<String, XMLStruct>, file_name: &str) -> io::Result<()> {
    let mut structs_string = String::new();

    // Build the structs as a string
    for (name, xml_struct) in structs.iter() {
        structs_string.push_str(&format!("#[derive(Debug, Serialize, Deserialize)]"));
        structs_string.push_str(&format!("\npub struct {} {{\n", name));

        for field in xml_struct.fields.iter() {
            if field.name == "base" {
                structs_string.push_str(&format!("    #[serde(flatten)]\n"));
            } else if field.field_type.starts_with("Option<") {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name));
            } else {
                structs_string.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
            }
            
            structs_string.push_str(&format!("    pub {}: {},\n", to_snake_case(&field.name), field.field_type));
        }

        structs_string.push_str("}\n\n");
    }

    // Write the string to the file
    let mut file = File::create(file_name)?;
    file.write_all(structs_string.as_bytes())?;

    Ok(())
}

fn element_definitions_to_file(element_definitions: &HashMap<String, String>, file_name: &str) -> io::Result<()> {
    let mut element_definitions_string = String::new();

    // Build the element definitions as a string
    for (name, typ) in element_definitions.iter() {
        element_definitions_string.push_str(&format!("pub struct {} {{\n", name));
        element_definitions_string.push_str(&format!("    pub {}: {},\n", to_snake_case(name), remove_prefix(typ)));
        element_definitions_string.push_str("}\n\n");
    }

    // Write the string to the file
    let mut file = File::create(file_name)?;
    file.write_all(element_definitions_string.as_bytes())?;

    Ok(())
}
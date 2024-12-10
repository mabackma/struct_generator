use struct_generator::create_structs::{XMLStruct, create_structs};
use struct_generator::string_utils::to_snake_case;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use quick_xml::Reader;

fn main() {
    let xsd_content = &read_xsd_file("schema.xsd").unwrap();
    let mut reader = Reader::from_str(xsd_content);
    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs

    create_structs(&mut reader, &mut structs);

    structs_to_file(&structs, "structs.rs").unwrap();
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
        structs_string.push_str(&format!("\npub struct {} {{\n", name));

        for field in xml_struct.fields.iter() {
            structs_string.push_str(&format!("    #[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name));
            structs_string.push_str(&format!("    pub {}: {},\n", to_snake_case(&field.name), field.field_type));
        }

        structs_string.push_str("}\n");
    }

    // Write the string to the file
    let mut file = File::create(file_name)?;
    file.write_all(structs_string.as_bytes())?;

    Ok(())
}
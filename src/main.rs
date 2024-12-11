use struct_generator::create_structs::{create_structs, XMLStruct};
use struct_generator::file_utils::{element_definitions_to_file, read_xsd_file, structs_to_file};

use std::collections::HashMap;
use std::fs;
use quick_xml::Reader;

fn main() {
    let folder_path = "./MetsatietostandardiskeematV33.03/V33"; // Path to your folder containing XSD files
    
    for entry in fs::read_dir(folder_path).unwrap() {
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
    }

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

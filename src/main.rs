use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::events::Event::{Start, End, Empty, Eof};
use quick_xml::name::QName;

#[derive(Debug, Clone)]
pub struct XMLField {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug)]
pub struct XMLStruct {
    name: String,
    pub fields: Vec<XMLField>,
}

impl Clone for XMLStruct {
    fn clone(&self) -> Self {
        XMLStruct {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}

impl XMLStruct {
    pub fn new() -> XMLStruct {
        XMLStruct {
            name: String::new(),
            fields: Vec::new(),
        }
    }
}

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

// Retrieve the element reference
fn element_references(e: &BytesStart<'_>) -> Option<String> {
    let e_ref = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"ref")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the ref attribute value as a string

    e_ref
}

// Retrieve the element name
fn element_names(e: &BytesStart<'_>) -> Option<String> {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    e_name
}

// Return the type of the element
fn element_types(e: &BytesStart<'_>) -> Option<String> {
    let e_type = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"type")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the type attribute value as a string

    e_type
}

// Check if the type is a Vec<>
fn is_element_vec(e: &BytesStart<'_>) -> bool {
    let e_max_occurs = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"maxOccurs")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the maxOccurs attribute value as a string

    if let Some(max_occurs) = e_max_occurs {
        if max_occurs == "unbounded"{
            return true;
        }

        if let Ok(num) = max_occurs.parse::<u32>() {
            if num > 1 {
                return true;
            }
        }
    }

    false
}

// Check if the type is optional
fn is_element_optional(e: &BytesStart<'_>) -> bool {
    let e_min_occurs = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"minOccurs")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the minOccurs attribute value as a string

    if let Some(min_occurs) = e_min_occurs {
        if min_occurs == "0" {
            return true;
        }
    }

    false
}

// Check if the type is a vector or optional
fn parse_type(e: &BytesStart<'_>, field_type: &mut String) {
    if is_element_vec(e) {
        if is_element_optional(e) {
            *field_type = format!("Option<Vec<{}>>", field_type);
        } else {
            *field_type = format!("Vec<{}>", field_type);
        }
    } else {
        if is_element_optional(e) {
            *field_type = format!("Option<{}>", field_type);
        }
    }
}

// Parse nested elements
fn create_structs(
    reader: &mut Reader<&[u8]>,
    structs: &mut HashMap<String, XMLStruct>,
) {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack to keep track of active structs
    let mut current_struct = XMLStruct::new(); // Root structure
    let mut current_name = String::new(); // Name of the current structure

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                if e.name() == QName(b"xs:complexType") {
                    if let Some(name) = element_names(e) {
                        current_name = name.clone();

                        // Create a new struct for this element
                        let mut new_struct = XMLStruct {
                            name: name.clone(),
                            fields: Vec::new(),
                        };

                        stack.push(new_struct.clone());
                    }
                }

                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") {
                    // If there's a parent struct, add this struct as a field to it
                    if let Some(parent_struct) = stack.last_mut() {
                        let mut name = element_names(e);

                        if name == None {
                            name = element_references(e);
                        }

                        name = Some(remove_prefix(&name.unwrap()));

                        if let Some(n) = name {
                            let mut field_type = n.clone();

                            if let Some(typ) = element_types(e) {
                                field_type = typ;
                            } 
                            
                            field_type = remove_prefix(&field_type);

                            parse_type(e, &mut field_type);

                            // Check if the field already exists
                            if !parent_struct.fields.iter().any(|field| field.name == n) {
                                // Add the field to the parent struct
                                parent_struct.fields.push(XMLField {
                                    name: n.clone(),
                                    field_type,
                                });
                            }
                        }
                    }
                }
            }
            Ok(Empty(ref e)) => {
                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") {
                    // If there's a parent struct, add this struct as a field to it
                    if let Some(parent_struct) = stack.last_mut() {
                        let mut name = element_names(e);

                        if name == None {
                            name = element_references(e);
                        }

                        name = Some(remove_prefix(&name.unwrap()));

                        if let Some(n) = name {
                            let mut field_type = n.clone();

                            if let Some(typ) = element_types(e) {
                                field_type = typ;
                            } 
                            
                            field_type = remove_prefix(&field_type);

                            parse_type(e, &mut field_type);

                            // Check if the field already exists
                            if !parent_struct.fields.iter().any(|field| field.name == n) {
                                // Add the field to the parent struct
                                parent_struct.fields.push(XMLField {
                                    name: n.clone(),
                                    field_type,
                                });
                            }
                        }
                    }
                }
            }
            Ok(End(ref e)) => {
                if e.name() == QName(b"xs:complexType") {

                    // Pop the current struct from the stack
                    if let Some(completed_struct) = stack.pop() {
                        if completed_struct.name != current_name {
                            panic!("XML structure mismatch: expected {}, found {}", completed_struct.name, current_name);
                        }

                        // Update the final struct with new fields or insert it if it doesn't exist
                        if let Some(existing_struct) = structs.get_mut(&completed_struct.name.clone()) {
                            // Merge fields: add only new unique fields
                            for field in completed_struct.fields {
                                if !existing_struct.fields.iter().any(|f| f.name == field.name) {
                                    existing_struct.fields.push(field.clone());
                                }
                            }
                        } else {
                            // No existing struct, insert the completed struct as it is
                            structs.insert(completed_struct.name.clone(), completed_struct.clone());
                        }
                    }
                }
            }
            Ok(Eof) => break,
            _ => {}
        }
    }
}

fn search_reference_type(ref_name: &str) -> Option<String> {
    // Search for the reference type in the schema
    Some("String from ref".to_string())
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

fn remove_prefix(name: &str) -> String {
    let mut parts = name.split(':').collect::<Vec<&str>>();

    if parts.len() > 1 {
        parts.remove(0);
        return parts.join("");
    }

    name.to_string()
}

fn to_snake_case(name: &str) -> String {
    let mut snake_case = String::new();
    let mut chars = name.chars();

    if let Some(first_char) = chars.next() {
        snake_case.push(first_char.to_ascii_lowercase());
    }

    for c in chars {
        if c.is_uppercase() {
            snake_case.push('_');
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }

    snake_case
}
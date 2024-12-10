use crate::string_utils::remove_prefix;

use std::collections::HashMap;
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

// Create structs from nested elements
pub fn create_structs(
    reader: &mut Reader<&[u8]>,
    structs: &mut HashMap<String, XMLStruct>,
) {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack to keep track of active structs
    let mut current_name = String::new(); // Name of the current structure
    let mut element_definitions: HashMap<String, String> = HashMap::new(); // Definitions for elements

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                if e.name() == QName(b"xs:complexType") {
                    if let Some(name) = element_name(e) {
                        current_name = name.clone();

                        // Create a new struct for this element
                        let new_struct = XMLStruct {
                            name: name.clone(),
                            fields: Vec::new(),
                        };

                        stack.push(new_struct.clone());
                    }
                }

                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") || e.name() == QName(b"xs:attribute") {
                    elements_and_groups(&mut stack, e, &mut element_definitions);
                }
            }
            Ok(Empty(ref e)) => {
                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") || e.name() == QName(b"xs:attribute") {
                    elements_and_groups(&mut stack, e, &mut element_definitions);
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

    attributes_first(structs);

    for (name, typ) in element_definitions.iter() {
        println!("{}: {}", name, typ);
    }
}

// Retrieve the element reference
fn element_reference(e: &BytesStart<'_>) -> Option<String> {
    let e_ref = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"ref")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the ref attribute value as a string

    e_ref
}

// Retrieve the element name
fn element_name(e: &BytesStart<'_>) -> Option<String> {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    e_name
}

// Return the type of the element
fn element_type(e: &BytesStart<'_>) -> Option<String> {
    let e_type = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"type")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the type attribute value as a string

    e_type
}

fn reference_type(ref_name: &str, element_definitions: &HashMap<String, String>) -> Option<String> {
    // Search for the reference type in the element definitions
    if let Some(typ) = element_definitions.get(ref_name) {
        println!("Found reference type: {}", typ);
        return Some(typ.clone());
    }

    Some("String".to_string())
}

// Add elements, groups, and attributes as fields to the struct
fn elements_and_groups(stack: &mut Vec<XMLStruct>, e: &BytesStart<'_>, element_definitions: &mut HashMap<String, String>) {
    // If there's a parent struct, add this struct as a field to it
    if let Some(parent_struct) = stack.last_mut() {
        let mut name = element_name(e);

        if name == None {
            name = element_reference(e);
        }

        if let Some(mut n) = name {
            let mut field_type = n.clone();

            if let Some(typ) = element_type(e) {
                field_type = typ;
                element_definitions.insert(n.clone(), field_type.clone());
            } else if let Some(typ) = reference_type(&n, element_definitions) {
                field_type = typ;   
            }

            n = remove_prefix(&n);
            field_type = remove_prefix(&field_type);

            parse_type(e, &mut field_type);

            if e.name() == QName(b"xs:attribute") {
                n = "@".to_string() + &n;
            }

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

// Move attributes to the beginning of the struct
fn attributes_first(structs: &mut HashMap<String, XMLStruct>) {
    for (_, xml_struct) in structs.iter_mut() {
        let mut attribute_fields: Vec<XMLField> = Vec::new();

        for field in xml_struct.fields.iter() {
            if field.name.starts_with('@') {
                attribute_fields.push(field.clone());
            }
        }

        // Add optional text field
        if xml_struct.fields.len() > 0 {
            attribute_fields.push(XMLField {
                name: "$text".to_string(),
                field_type: "Option<String>".to_string(),
            });
        }

        for field in xml_struct.fields.iter() {
            if !field.name.starts_with('@') {
                attribute_fields.push(field.clone());
            }
        }

        *xml_struct = XMLStruct {
            name: xml_struct.name.clone(),
            fields: attribute_fields,
        };
    }
}
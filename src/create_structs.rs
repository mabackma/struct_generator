use crate::element_utils::{element_name, element_reference, element_type, extension_type, parse_type, reference_type};
use crate::string_utils::{remove_prefix, slice_contents};

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
    element_definitions: &mut HashMap<String, String>,
    content: &str,
) {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack to keep track of active structs
    let mut current_name = String::new(); // Name of the current structure
    let mut anonymous_complex_types: Vec<String> = Vec::new(); // Anonymous complex types

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                if e.name() == QName(b"xs:complexType") || e.name() == QName(b"xs:simpleType") {
                    if let Some(name) = element_name(e) {
                        current_name = name.clone();

                        // Create a new struct for this element
                        let new_struct = XMLStruct {
                            name: current_name.clone(),
                            fields: Vec::new(),
                        };

                        stack.push(new_struct.clone());
                    }
                }

                if e.name() == QName(b"xs:extension") || e.name() == QName(b"xs:restriction") {
                    add_extension_fields(&mut stack, e);
                }

                if e.name() == QName(b"xs:element") {
                    add_element_definition(e, element_definitions, content, &mut stack, &mut current_name, &mut anonymous_complex_types);
                }

                if e.name() == QName(b"xs:group") {
                    add_group_definition(e, content, structs, &mut current_name, &mut anonymous_complex_types);
                }

                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") || e.name() == QName(b"xs:attribute") {
                    elements_and_groups(&mut stack, e, &element_definitions, &mut anonymous_complex_types);
                }
            }
            Ok(Empty(ref e)) => {
                if e.name() == QName(b"xs:extension") || e.name() == QName(b"xs:restriction") {
                    add_extension_fields(&mut stack, e);
                }

                if e.name() == QName(b"xs:element") {
                    add_element_definition(e, element_definitions, content, &mut stack, &mut current_name, &mut anonymous_complex_types);
                }

                if e.name() == QName(b"xs:element") || e.name() == QName(b"xs:group") || e.name() == QName(b"xs:attribute") {
                    elements_and_groups(&mut stack, e, &element_definitions, &mut anonymous_complex_types);
                }
            }
            Ok(End(ref e)) => {
                if e.name() == QName(b"xs:complexType") || e.name() == QName(b"xs:simpleType") {

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
}

// Add element definitions to the hashmap
pub fn add_element_definition(e: &BytesStart<'_>, element_definitions: &mut HashMap<String, String>, content: &str, stack: &mut Vec<XMLStruct>, current_name: &mut String, anonymous_complex_types: &mut Vec<String>) {
    let name = element_name(e);
    let typ = element_type(e);

    if let Some(n) = name {
        if let Some(t) = typ {
            if element_definitions.contains_key(&n) && element_definitions[&n] != t {
                println!("Existing definition {}: {} -> {}", n, element_definitions[&n], t);
            }

            element_definitions.insert(n, t);
        } else {
            if let Some(_) = slice_contents(content, "element",&n) {
                println!("Anonymous complex type: {}", n);
                // Update the current name when creating a new struct
                *current_name = n.clone();

                anonymous_complex_types.push(n.clone());

                // Create a struct for the anonymous complex type
                let new_struct = XMLStruct {
                    name: n.clone(),
                    fields: Vec::new(),
                };

                stack.push(new_struct.clone());

                // Create element definition using the name of the element and "Type"
                element_definitions.insert(n.clone() + "Type", n);
            } else {
                //println!("No type for element: {}", n);
            }
        }
    }
}

// Add group definitions directly to the final structs
fn add_group_definition(e: &BytesStart<'_>, content: &str, structs: &mut HashMap<String, XMLStruct>,  current_name: &mut String, anonymous_complex_types: &mut Vec<String>) {

    if let Some(group_slice) = slice_contents(content, "group", &element_name(e).unwrap_or("".to_string())) {
        let mut group_reader = Reader::from_str(&group_slice);
        let mut group_types = HashMap::new();

        loop {
            match group_reader.read_event() {
                Ok(Start(ref ge)) => {
                    if ge.name() == QName(b"xs:element") {
                        if let Some(r) = element_name(ge) {
                            let mut field_type = r.clone();

                            // Check if the field is a vector or optional
                            parse_type(ge, &mut field_type);

                            group_types.insert(r,field_type);
                        }
                    }
                },
                Ok(Empty(ref ge)) => {
                    if ge.name() == QName(b"xs:element") {
                        if let Some(r) = element_reference(ge) {
                            let mut field_type = r.clone();

                            // Check if the field is a vector or optional
                            parse_type(ge, &mut field_type);

                            group_types.insert(r,field_type);
                        }
                    }
                },
                Ok(End(ref ge)) => {
                    if ge.name() == QName(b"xs:group") {
                        let name = element_name(e).unwrap_or("".to_string());
    
                        // Create a struct for the group type
                        let new_struct = XMLStruct {
                            name: name.clone(),
                            fields: group_types.iter().map(|t| XMLField {
                                name: t.0.to_string(),
                                field_type: t.1.to_string(),
                            }).collect(),
                        };
                        
                        structs.insert(name.clone(), new_struct.clone());

                        break;
                    }
                },
                Ok(Eof) => break,
                _ => {}
            }
        }
    }
}

// Add elements, groups, and attributes as fields to the struct
fn elements_and_groups(stack: &mut Vec<XMLStruct>, e: &BytesStart<'_>, element_definitions: &HashMap<String, String>, anonymous_complex_types: &mut Vec<String>) {

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
            } else if let Some(typ) = reference_type(&n, element_definitions) {
                field_type = typ;   
            }

            n = remove_prefix(&n);
            field_type = remove_prefix(&field_type);

            // Define vector and optional types
            parse_type(e, &mut field_type);

            if e.name() == QName(b"xs:attribute") {
                n = "@".to_string() + &n;
            }

            // Check if the field already exists
            if !parent_struct.fields.iter().any(|field| field.name == n) && !anonymous_complex_types.contains(&n) {

                // Add the field to the parent struct
                parent_struct.fields.push(XMLField {
                    name: n.clone(),
                    field_type,
                });
            }
        }
    }
}

// Add extension fields to the struct
fn add_extension_fields(stack: &mut Vec<XMLStruct>, e: &BytesStart<'_>) {

    // If there's a parent struct, add this struct as a field to it
    if let Some(parent_struct) = stack.last_mut() {
        let mut field_type = "".to_string();

        if let Some(typ) = extension_type(e) {
            field_type = remove_prefix(&typ);
        }

        // Add the field to the parent struct
        parent_struct.fields.push(XMLField {
            name: "base".to_string(),
            field_type,
        });
    }
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

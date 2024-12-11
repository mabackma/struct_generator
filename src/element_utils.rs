use std::collections::HashMap;
use quick_xml::events::BytesStart;
use quick_xml::name::QName;

// Retrieve the element reference
pub fn element_reference(e: &BytesStart<'_>) -> Option<String> {
    let e_ref = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"ref")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the ref attribute value as a string

    e_ref
}

// Retrieve the element name
pub fn element_name(e: &BytesStart<'_>) -> Option<String> {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string
    
    e_name
}

// Return the type of the element
pub fn element_type(e: &BytesStart<'_>) -> Option<String> {
    let e_type = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"type")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the type attribute value as a string

    e_type
}

// Return the type of the extension
pub fn extension_type(e: &BytesStart<'_>) -> Option<String> {
    let e_base = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"base")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the base attribute value as a string

    e_base
}

// Retrieve the type of the reference
pub fn reference_type(ref_name: &str, element_definitions: &HashMap<String, String>) -> Option<String> {
    // Search for the reference type in the element definitions
    if let Some(typ) = element_definitions.get(ref_name) {        
        return Some(typ.clone());
    }

    // TODO: Implement a way to get the reference type from the XSD file 
    // refferred to in the beginning of the file with the `xs:import` tag

    //Some(ref_name.to_string())
    Some("String".to_string())
}


// Check if the type is a vector or optional
pub fn parse_type(e: &BytesStart<'_>, field_type: &mut String) {
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
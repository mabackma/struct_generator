use crate::file_utils::RUST_TYPES;
use crate::string_utils::{capitalize_first, handle_prefix, XSD_TO_RUST};

use std::collections::HashMap;
use quick_xml::events::{BytesEnd, BytesStart};
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

    if e_type.clone().unwrap_or("".to_string()).starts_with("xs:") {
        let new_type = e_type.clone().unwrap_or("".to_string()).replace("xs:", "");
        let new_type = XSD_TO_RUST.get(&new_type);

        return Some(new_type.unwrap().to_string());
    }

    e_type
}

// Return the type of the extension
pub fn extension_type(e: &BytesStart<'_>) -> Option<String> {
    let e_base = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"base")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the base attribute value as a string

    e_base
}

// Return the types in the union
pub fn union_members(e: &BytesStart<'_>) -> Option<String> {
    let e_base = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"memberTypes")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the base attribute value as a string

    e_base
}

// Retrieve the type of the reference
pub fn reference_type(
    ref_name: &str, 
    element_definitions: &HashMap<String, String>, 
    prefixes: &mut HashMap<String, String>
) -> Option<String> {

    let complete_name = handle_prefix(ref_name, prefixes);

    // Search for the reference type in the element definitions
    // If found, return the type
    if let Some(typ) = element_definitions.get(&complete_name) {
        return Some(typ.clone());
    } 

    Some(complete_name.to_string())
}

// Check if the type is a vector or optional
pub fn parse_type(
    e: &BytesStart<'_>, 
    field_type: &mut String
) {
    *field_type = if let Some(ft) = XSD_TO_RUST.get(&field_type) {
        ft.to_string()
    } else {
        field_type.to_string()
    }; 

    if field_type.starts_with("xlink") || field_type.starts_with("Xlink") {
        *field_type = "String".to_string();
    }

    if field_type.contains("-") {
        *field_type = field_type.replace("-", "");
    }
    
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




// The following functions will be used incase we need to generate structs from the gml and stanford2010 folders
/* pub fn is_complex_or_simple(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:simpleType"), 
        QName(b"xs:complexType"), 
        QName(b"xsd:simpleType"),
        QName(b"xsd:complexType"), 
        QName(b"simpleType"), 
        QName(b"complexType")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_complex_or_simple_end(e: &BytesEnd<'_>) -> bool {

    let types = vec![
        QName(b"xs:simpleType"), 
        QName(b"xs:complexType"), 
        QName(b"xsd:simpleType"),
        QName(b"xsd:complexType"), 
        QName(b"simpleType"), 
        QName(b"complexType")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_extension_or_restriction(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:extension"), 
        QName(b"xs:restriction"), 
        QName(b"xsd:extension"),
        QName(b"xsd:restriction"), 
        QName(b"extension"), 
        QName(b"restriction")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_union(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:union"), 
        QName(b"xsd:union"), 
        QName(b"union")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_element(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:element"), 
        QName(b"xsd:element"), 
        QName(b"element")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_group(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:group"), 
        QName(b"xsd:group"), 
        QName(b"group")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
}

pub fn is_group_or_element_or_attribute(e: &BytesStart<'_>) -> bool {
    is_group(e) || is_element(e) || is_attribute(e)
}

pub fn is_attribute(e: &BytesStart<'_>) -> bool {

    let types = vec![
        QName(b"xs:attribute"), 
        QName(b"xsd:attribute"), 
        QName(b"attribute")
    ];

    if types.contains(&e.name()) {
        true
    } else {
        false
    }
} */
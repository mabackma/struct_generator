use std::fs::File;
use std::io::{self, Read};
use quick_xml::events::BytesStart;
use quick_xml::{Reader, events::{Event, BytesText}};
use quick_xml::events::Event::{Start, End, Empty, Eof, Text, Comment};
use quick_xml::name::QName;

fn main() {
    let xsd_content = &read_xsd_file("schema.xsd").unwrap();
    let mut reader = Reader::from_str(xsd_content);

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                if e.name() == QName(b"xs:element") {
                    parse_element(e);
                } 

                if e.name() == QName(b"xs:complexType") || e.name() == QName(b"xs:sequence") {
                    complex_types(&mut reader, e);
                }
            }
            Ok(Empty(ref e)) => {
                if e.name() == QName(b"xs:element") {
                    parse_element(e);
                } 

                if e.name() == QName(b"xs:complexType") || e.name() == QName(b"xs:sequence") {
                    complex_types(&mut reader, e);
                }
            },
            Ok(Eof) => break,
            _ => {}
        }
    }
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

fn parse_element(e: &BytesStart<'_>) {
    element_references(e);
    element_names_and_types(e);
    is_element_optional(e);
    println!();
}

fn element_references(e: &BytesStart<'_>) {
    let e_ref = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"ref")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the ref attribute value as a string
    
    // If the element has a ref attribute, it cannot have a type attribute
    if let Some(ref r) = e_ref {
        print!("ref: {}", r);
        print!(" -> type: String");
    }
}

fn element_names_and_types(e: &BytesStart<'_>) {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    let e_type = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"type")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok());

    if let Some(ref name) = e_name {
        print!("name: {}", name);
    }

    if let Some(type_name) = e_type {
        print!(" -> type: {}", type_name);
    } 
}

fn is_element_optional(e: &BytesStart<'_>) -> bool {
    let e_min_occurs = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"minOccurs")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the minOccurs attribute value as a string

    if let Some(min_occurs) = e_min_occurs {
        if min_occurs == "0" {
            println!(" (optional)");
            return true;
        }
    }

    println!(" (required)");
    false
}

// Parse xs:complexType elements
fn complex_types(reader: &mut Reader<&[u8]>, e: &BytesStart<'_>) {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    if let Some(ref name) = e_name {
        print!("\nComplexType: {} ", name);
    }

    // Now handle the nested xs:attribute tags inside the complexType
    // Read nested elements inside xs:complexType
    loop {
        match reader.read_event() {
            Ok(Start(ref child)) => {
                if child.name() == QName(b"xs:element") {
                    parse_element(child);
                }

                if child.name() == QName(b"xs:complexType") || child.name() == QName(b"xs:sequence") {
                    complex_types(reader, child);
                }
            }
            Ok(End(ref child)) => {
                if child.name() == QName(b"xs:complexType") || child.name() == QName(b"xs:sequence") {
                    break; // End of the complexType, stop processing nested elements
                }
            }
            Ok(Eof) => break,
            _ => {}
        }
    }
}

use std::fs::File;
use std::io::{self, Read};
use std::vec;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::events::Event::{Start, End, Empty, Eof};
use quick_xml::name::QName;
use once_cell::sync::Lazy;

// Lazily initialize the PARENTS variable
static PARENTS: Lazy<Vec<QName<'static>>> = Lazy::new(|| {
    vec![
        QName(b"xs:complexType"),
        QName(b"xs:sequence"),
        QName(b"xs:choice"),
    ]
});


fn main() {
    let xsd_content = &read_xsd_file("schema.xsd").unwrap();
    let mut reader = Reader::from_str(xsd_content);

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                if e.name() == QName(b"xs:element") {
                    parse_element(e);
                } 

                if PARENTS.contains(&e.name()) {
                    complex_types(&mut reader, e);
                }
            }
            Ok(Empty(ref e)) => {
                if e.name() == QName(b"xs:element") {
                    parse_element(e);
                } 

                if PARENTS.contains(&e.name()) {
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
    let mut name = element_references(e);
    let mut e_type = None;

    if name.is_none() {
        name = element_names(e);
        e_type = element_types(e);
    }

    if is_element_vec(e) {
        if is_element_optional(e) {
            print!("{}: Option<Vec<{}>>,", name.clone().unwrap(), e_type.unwrap_or(name.unwrap()));
        } else {
            print!("{}: Vec<{}>,", name.clone().unwrap(), e_type.unwrap_or(name.unwrap()));
        }
    } else {
        if is_element_optional(e) {
            print!("{}: Option<{}>,", name.clone().unwrap(), e_type.unwrap_or(name.unwrap()));
        } else {
            print!("{}: {},", name.clone().unwrap(), e_type.unwrap_or(name.unwrap()));
        }
    }

    println!();
}

fn element_references(e: &BytesStart<'_>) -> Option<String> {
    let e_ref = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"ref")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the ref attribute value as a string

    if let Some(_) = e_ref {
        print!("ref: ");
    }

    e_ref
}

fn element_names(e: &BytesStart<'_>) -> Option<String> {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    if let Some(_) = e_name {
        print!("name: ");
    }

    e_name
}

fn element_types(e: &BytesStart<'_>) -> Option<String> {
    let e_type = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"type")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the type attribute value as a string

    e_type
}

// Check if the element is a Vec<>
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

// Check if the element is optional
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

// Parse xs:complexType elements
fn complex_types(reader: &mut Reader<&[u8]>, e: &BytesStart<'_>) {
    let e_name = e.attributes().filter_map(|a| a.ok())
        .find(|a| a.key == QName(b"name")) 
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok()); // Extract the name attribute value as a string

    if let Some(ref name) = e_name {
        println!("\npub struct {} {{", name);
    }

    // Now handle the nested xs:attribute tags inside the complexType
    // Read nested elements inside xs:complexType
    loop {
        match reader.read_event() {
            Ok(Start(ref child)) => {
                if child.name() == QName(b"xs:element") {
                    parse_element(child);
                }

                if PARENTS.contains(&child.name()) {
                    complex_types(reader, child);
                }
            },
            Ok(Empty(ref child)) => {
                if child.name() == QName(b"xs:element") {
                    parse_element(child);
                }

                if PARENTS.contains(&child.name()) {
                    complex_types(reader, child);
                }
            }
            Ok(End(ref child)) => {
                if PARENTS.contains(&child.name()) {
                    break; // End of the complexType, stop processing nested elements
                }
            }
            Ok(Eof) => break,
            _ => {}
        }
    }
}
    
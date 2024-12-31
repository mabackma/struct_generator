use std::collections::HashMap;
use phf::{Map, phf_map};

pub static XSD_TO_RUST: Map<&'static str, &str> = phf_map! {
    "boolean" => "bool",
    "decimal" => "f64",
    "float" => "f32",
    "double" => "f64",
    "integer" => "i32",
    "positiveInteger" => "u32",
    "nonPositiveInteger" => "i32",
    "negativeInteger" => "i32",
    "nonNegativeInteger" => "u32",
    "byte" => "i8",
    "short" => "i16",
    "int" => "i32",
    "long" => "i64",
    "unsignedByte" => "u8",
    "unsignedShort" => "u16",
    "unsignedInt" => "u32",
    "unsignedLong" => "u64",
    "string" => "String",
    "normalizedString" => "String",
    "token" => "String",
    "language" => "String",
    "Name" => "String",
    "NCName" => "String",
    "ID" => "String",
    "IDREF" => "String",
    "ENTITY" => "String",
    "anyURI" => "String",
    "date" => "chrono::NaiveDate",
    "time" => "chrono::NaiveTime",
    "dateTime" => "chrono::NaiveDateTime",
    "duration" => "std::time::Duration",
    "gYear" => "chrono::NaiveDate",
    "gMonth" => "chrono::NaiveDate",
    "gDay" => "chrono::NaiveDate",
    "gYearMonth" => "chrono::NaiveDate",
    "gMonthDay" => "chrono::NaiveDate",
    "dateTimeStamp" => "chrono::NaiveDateTime",
    "base64Binary" => "Vec<u8>",
    "hexBinary" => "Vec<u8>",
    "anySimpleType" => "String",
    "DateYYYY-MMOrYYYY-MM-DDType" => "chrono::NaiveDate",
    "Point" => "Point<f64>",
    "Polygon" => "Polygon<f64>",
    "MultiPolygon" => "MultiPolygon<f64>",
    "pointProperty" => "Point<f64>",
    "polygonProperty" => "Polygon<f64>",
};

pub fn handle_prefix(
    name: &str, 
    prefixes: &mut HashMap<String, String>
) -> String {
    let parts = name.split(':').collect::<Vec<&str>>();
    let prefix = parts[0];

    if parts.len() > 1 {
        let complete_name = parts.join("");
        let complete_name = capitalize_first(&complete_name).to_string();

        prefixes.insert(complete_name.clone(), prefix.to_string());

        return complete_name;
    }

    name.to_string()
}

pub fn capitalize_first(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_snake_case(name: &str) -> String {
    // Handle the case where the name is "type"
    if name == "Type" || name == "@type" {
        return "r#type".to_string();
    }

    // Handle the case where the name is all uppercase
    if name.chars().all(char::is_uppercase) {
        return name.to_ascii_lowercase();
    }
    
    let mut snake_case = String::new();

    // Use a unified iterator type for `chars`
    let mut chars: Box<dyn Iterator<Item = char>> = if name.starts_with('@') {
        Box::new(name.chars().skip(1))
    } else {
        Box::new(name.chars())
    };

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

pub fn slice_contents(
    content: &str, 
    tag: &str, 
    name: &str
) -> Option<String> {
    
    // Find the start and end positions of the element
    let start_tag = format!("<xs:{} name=\"{}\">", tag, name);
    let end_tag = format!("</xs:{}>", tag);

    // Find the start position of the element
    if let Some(start_pos) = content.find(&start_tag) {
        // Find the first occurrence of the end tag after the start position
        if let Some(relative_end_pos) = content[start_pos..].find(&end_tag) {
            let end_pos = start_pos + relative_end_pos + end_tag.len();

            // Slice the content including the start and end tags
            let element_content = &content[start_pos..end_pos];
            return Some(element_content.trim().to_string());
        }
    }

    None
}
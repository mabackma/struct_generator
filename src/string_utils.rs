pub fn remove_prefix(name: &str) -> String {
    let mut parts = name.split(':').collect::<Vec<&str>>();

    if parts.len() > 1 {
        parts.remove(0);
        return parts.join("");
    }

    name.to_string()
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

// Slice the content of an element using find and rfind
pub fn slice_element_contents(content: &str, element_name: &str) -> Option<String> {

    // Find the start and end positions for the <xs:element name="CallForOffers">
    let start_tag = format!("<xs:element name=\"{}\">", element_name);
    let end_tag = "</xs:element>";

    // Find the start and end positions of the element
    if let Some(start_pos) = content.find(&start_tag) {
        if let Some(end_pos) = content.rfind(end_tag) {
            // Slice the content between the start and end positions
            let start_pos = start_pos + start_tag.len();
            let element_content = &content[start_pos..end_pos];
            Some(element_content.trim().to_string())
        } else {
            None
        }
    } else {
        None
    }
}
pub fn remove_prefix(name: &str) -> String {
    let mut parts = name.split(':').collect::<Vec<&str>>();

    if parts.len() > 1 {
        parts.remove(0);
        return parts.join("");
    }

    name.to_string()
}

pub fn to_snake_case(name: &str) -> String {
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
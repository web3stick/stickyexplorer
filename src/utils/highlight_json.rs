// utils/highlight_json.rs
// =========================================
// JSON syntax highlighter for display
// =========================================

/// Simple JSON syntax highlighter that wraps tokens in HTML spans
/// with CSS classes for coloring. Returns HTML string.
pub fn highlight_json(json: &str) -> String {
    let mut result = String::with_capacity(json.len() * 2);
    let mut chars = json.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(c) = chars.next() {
        if escape_next {
            result.push(c);
            escape_next = false;
            continue;
        }

        if c == '\\' && in_string {
            result.push(c);
            escape_next = true;
            continue;
        }

        if c == '"' {
            if !in_string {
                in_string = true;
                result.push_str("<span class=\"json-string\">\"");
            } else {
                in_string = false;
                result.push_str("\"</span>");
            }
        } else if in_string {
            result.push(c);
        } else if c == ':' {
            result.push_str(":&nbsp;");
        } else if c == '{' || c == '[' {
            result.push(c);
        } else if c == '}' || c == ']' {
            result.push(c);
        } else if c == ',' {
            result.push_str(",<br>");
        } else if c.is_whitespace() {
            result.push(c);
        } else {
            // Numbers, booleans, null
            let mut token = String::from(c);
            loop {
                match chars.peek() {
                    Some(&next_c)
                        if !next_c.is_whitespace()
                            && next_c != ','
                            && next_c != '}'
                            && next_c != ']' =>
                    {
                        token.push(next_c);
                        chars.next();
                    }
                    _ => break,
                }
            }
            let class = if token == "null" {
                "json-null"
            } else if token == "true" || token == "false" {
                "json-boolean"
            } else {
                "json-number"
            };
            result.push_str(&format!("<span class=\"{}\">{}</span>", class, token));
        }
    }

    result
}
// =========================================
// copyright 2026 by sleet.near

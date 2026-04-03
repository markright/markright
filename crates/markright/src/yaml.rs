/// Minimal YAML parser for front matter.
///
/// Supports: scalars (string, number, boolean, null), flat/nested maps,
/// and simple sequences. No anchors, no flow syntax, no multi-line strings.
#[cfg(feature = "serde")]
pub fn parse_yaml(input: &str) -> serde_json::Value {
    let lines: Vec<&str> = input.lines().collect();
    let (val, _) = parse_value(&lines, 0, 0);
    val
}

#[cfg(feature = "serde")]
fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

#[cfg(feature = "serde")]
fn parse_value(lines: &[&str], start: usize, min_indent: usize) -> (serde_json::Value, usize) {
    if start >= lines.len() {
        return (serde_json::Value::Null, start);
    }

    let line = lines[start];
    let trimmed = line.trim();

    // Empty or comment line -- skip
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return parse_value(lines, start + 1, min_indent);
    }

    let indent = indent_of(line);
    if indent < min_indent {
        return (serde_json::Value::Null, start);
    }

    // Sequence item
    if trimmed.starts_with("- ") || trimmed == "-" {
        return parse_sequence(lines, start, indent);
    }

    // Map entry (key: value)
    if trimmed.contains(": ") || trimmed.ends_with(':') {
        return parse_map(lines, start, indent);
    }

    // Bare scalar
    (parse_scalar(trimmed), start + 1)
}

#[cfg(feature = "serde")]
fn parse_map(lines: &[&str], start: usize, map_indent: usize) -> (serde_json::Value, usize) {
    let mut map = serde_json::Map::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }

        let indent = indent_of(line);
        if indent != map_indent {
            break;
        }

        if let Some(colon_pos) = trimmed.find(": ") {
            let key = trimmed[..colon_pos].trim().to_string();
            let val_str = trimmed[colon_pos + 2..].trim();
            if val_str.is_empty() {
                // Block value on next lines
                let child_indent = if i + 1 < lines.len() {
                    let next = lines[i + 1];
                    if next.trim().is_empty() {
                        map_indent + 2
                    } else {
                        indent_of(next)
                    }
                } else {
                    map_indent + 2
                };
                let (val, next_i) = parse_value(lines, i + 1, child_indent);
                map.insert(key, val);
                i = next_i;
            } else {
                map.insert(key, parse_scalar(val_str));
                i += 1;
            }
        } else if trimmed.ends_with(':') {
            let key = trimmed.strip_suffix(':').unwrap().trim().to_string();
            let child_indent = if i + 1 < lines.len() {
                let next = lines[i + 1];
                if next.trim().is_empty() {
                    map_indent + 2
                } else {
                    indent_of(next)
                }
            } else {
                map_indent + 2
            };
            let (val, next_i) = parse_value(lines, i + 1, child_indent);
            map.insert(key, val);
            i = next_i;
        } else {
            break;
        }
    }

    (serde_json::Value::Object(map), i)
}

#[cfg(feature = "serde")]
fn parse_sequence(lines: &[&str], start: usize, seq_indent: usize) -> (serde_json::Value, usize) {
    let mut arr = Vec::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }

        let indent = indent_of(line);
        if indent != seq_indent {
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("- ") {
            let rest = rest.trim();
            if rest.is_empty() {
                let (val, next_i) = parse_value(lines, i + 1, indent + 2);
                arr.push(val);
                i = next_i;
            } else if rest.contains(": ") || rest.ends_with(':') {
                let (rebuilt, lines_consumed) = rebuild_dedented(lines, i, rest);
                let (val, _) = parse_map(&rebuilt, 0, 0);
                arr.push(val);
                i += lines_consumed;
            } else {
                arr.push(parse_scalar(rest));
                i += 1;
            }
        } else if trimmed == "-" {
            let (val, next_i) = parse_value(lines, i + 1, indent + 2);
            arr.push(val);
            i = next_i;
        } else {
            break;
        }
    }

    (serde_json::Value::Array(arr), i)
}

/// Rebuild lines for an inline sequence-map item: "- key: val" -> "key: val"
/// Returns (dedented_lines, original_lines_consumed).
#[cfg(feature = "serde")]
fn rebuild_dedented<'a>(
    lines: &[&'a str],
    idx: usize,
    first_content: &'a str,
) -> (Vec<&'a str>, usize) {
    let mut result = vec![first_content];
    let base_indent = indent_of(lines[idx]);
    let child_indent = base_indent + 2;
    let mut j = idx + 1;
    while j < lines.len() {
        let l = lines[j];
        if l.trim().is_empty() {
            j += 1;
            continue;
        }
        if indent_of(l) >= child_indent {
            result.push(l.trim());
            j += 1;
        } else {
            break;
        }
    }
    (result, j - idx)
}

#[cfg(feature = "serde")]
fn parse_scalar(s: &str) -> serde_json::Value {
    // Unquote strings
    if s.len() >= 2
        && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
    {
        return serde_json::Value::String(s[1..s.len() - 1].to_string());
    }

    match s {
        "true" | "True" | "yes" | "Yes" => serde_json::Value::Bool(true),
        "false" | "False" | "no" | "No" => serde_json::Value::Bool(false),
        "null" | "Null" | "~" => serde_json::Value::Null,
        _ => {
            if let Ok(n) = s.parse::<i64>() {
                serde_json::Value::Number(n.into())
            } else if let Ok(f) = s.parse::<f64>() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or_else(|| serde_json::Value::String(s.to_string()))
            } else {
                serde_json::Value::String(s.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_map() {
        let v = parse_yaml("title: Hello\nauthor: World\ncount: 42\npublished: true\n");
        assert_eq!(v["title"], "Hello");
        assert_eq!(v["author"], "World");
        assert_eq!(v["count"], 42);
        assert_eq!(v["published"], true);
    }

    #[test]
    fn nested_map() {
        let v = parse_yaml("author:\n  name: SC\n  email: sc@example.com\ntitle: Test\n");
        assert_eq!(v["author"]["name"], "SC");
        assert_eq!(v["author"]["email"], "sc@example.com");
        assert_eq!(v["title"], "Test");
    }

    #[test]
    fn simple_list() {
        let v = parse_yaml("tags:\n  - one\n  - two\n  - three\n");
        let tags = v["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "one");
        assert_eq!(tags[1], "two");
    }

    #[test]
    fn quoted_strings() {
        let v = parse_yaml("title: \"Hello: World\"\nalt: 'single'\n");
        assert_eq!(v["title"], "Hello: World");
        assert_eq!(v["alt"], "single");
    }

    #[test]
    fn null_and_empty() {
        let v = parse_yaml("a: null\nb: ~\nc:\n");
        assert!(v["a"].is_null());
        assert!(v["b"].is_null());
    }

    #[test]
    fn booleans() {
        let v = parse_yaml("a: true\nb: false\nc: yes\nd: no\n");
        assert_eq!(v["a"], true);
        assert_eq!(v["b"], false);
        assert_eq!(v["c"], true);
        assert_eq!(v["d"], false);
    }

    #[test]
    fn real_front_matter() {
        let v = parse_yaml(
            "title: MarkRight Feature Tour\nauthor: MarkRight Team\nsyntax: markright\n",
        );
        assert_eq!(v["title"], "MarkRight Feature Tour");
        assert_eq!(v["syntax"], "markright");
    }

    #[test]
    fn single_char_quote_no_panic() {
        let v = parse_yaml("a: \"\nb: '\n");
        assert_eq!(v["a"], "\"");
        assert_eq!(v["b"], "'");
    }

    #[test]
    fn sequence_of_maps() {
        let v = parse_yaml("items:\n  - name: Alice\n    age: 30\n  - name: Bob\n    age: 25\n");
        let items = v["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["name"], "Alice");
        assert_eq!(items[0]["age"], 30);
        assert_eq!(items[1]["name"], "Bob");
        assert_eq!(items[1]["age"], 25);
    }
}

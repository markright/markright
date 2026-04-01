use std::fs;
use std::path::{Path, PathBuf};

fn all_fixtures() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut files = Vec::new();
    collect_recursive(&root, &mut files);
    files.sort();
    files
}

fn collect_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_recursive(&path, out);
        } else if path.to_str().is_some_and(|s| s.ends_with(".test.right")) {
            out.push(path);
        }
    }
}

fn parse_and_format(input: &str) -> String {
    let bump = markright::Bump::new();
    let doc = markright::parse(input, &bump);
    markright::to_string(&doc)
}

fn parse_and_html(input: &str) -> String {
    let bump = markright::Bump::new();
    let doc = markright::parse(input, &bump);
    markright::to_html(&doc)
}

/// Normalize insignificant whitespace in HTML for comparison.
/// Leading spaces inside elements (like `<p> text`) have no visual effect.
fn normalize_html(html: &str) -> String {
    let parts: Vec<&str> = html.split('>').collect();
    let mut result = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push('>');
            result.push_str(part.trim_start_matches(' '));
        } else {
            result.push_str(part);
        }
    }
    result
}

#[test]
fn formatter_idempotent() {
    for path in all_fixtures() {
        let input = fs::read_to_string(&path).unwrap();
        let once = parse_and_format(&input);
        let twice = parse_and_format(&once);
        assert_eq!(
            once,
            twice,
            "\nFormatter not idempotent for: {}\nFirst:\n{once}\nSecond:\n{twice}",
            path.display()
        );
    }
}

#[test]
fn format_preserves_html() {
    for path in all_fixtures() {
        let input = fs::read_to_string(&path).unwrap();
        let html_original = normalize_html(&parse_and_html(&input));
        let formatted = parse_and_format(&input);
        let html_formatted = normalize_html(&parse_and_html(&formatted));
        assert_eq!(
            html_original,
            html_formatted,
            "\nFormatting changed HTML for: {}\nOriginal HTML:\n{html_original}\nFormatted HTML:\n{html_formatted}",
            path.display()
        );
    }
}

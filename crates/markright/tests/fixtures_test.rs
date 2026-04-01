use std::path::{Path, PathBuf};
use std::{env, fs};

fn collect(subdir: &str) -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(subdir);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| Some(e.ok()?.path()))
        .filter(|p| p.to_str().is_some_and(|s| s.ends_with(".test.right")))
        .collect();
    files.sort();
    files
}

fn check(path: &Path) {
    let input = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let json_path = path.with_extension("ast.json");

    let bump = markright::Bump::new();
    let doc = markright::parse(&input, &bump);
    let actual = serde_json::to_value(&doc).unwrap();

    if env::var("UPDATE_FIXTURES").is_ok() {
        fs::write(&json_path, serde_json::to_string_pretty(&actual).unwrap()).unwrap();
        return;
    }

    let expected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&json_path).unwrap_or_else(|e| panic!("{}: {e}", json_path.display())),
    )
    .unwrap_or_else(|e| panic!("bad JSON {}: {e}", json_path.display()));

    assert_eq!(
        actual,
        expected,
        "\nFixture: {}\nExpected:\n{}\nActual:\n{}\n",
        path.display(),
        serde_json::to_string_pretty(&expected).unwrap(),
        serde_json::to_string_pretty(&actual).unwrap(),
    );
}

macro_rules! fixture_test {
    ($name:ident, $dir:literal) => {
        #[test]
        fn $name() {
            let files = collect($dir);
            assert!(!files.is_empty());
            for f in &files {
                check(f);
            }
        }
    };
}

fixture_test!(loose, "");
fixture_test!(admonitions, "admonitions");
fixture_test!(auto_numbering, "auto-numbering");
fixture_test!(block_ids, "block-ids");
fixture_test!(blockquotes, "blockquotes");
fixture_test!(citations, "citations");
fixture_test!(code_blocks, "code-blocks");
fixture_test!(cross_construct, "cross-construct");
fixture_test!(emphasis, "emphasis");
fixture_test!(empty_docs, "empty-docs");
fixture_test!(escapes, "escapes");
fixture_test!(fenced_divs, "fenced-divs");
fixture_test!(footnotes, "footnotes");
fixture_test!(front_matter, "front-matter");
fixture_test!(hard_breaks, "hard-breaks");
fixture_test!(headings, "headings");
fixture_test!(html_rejection, "html-rejection");
fixture_test!(inline_code_math, "inline-code-math");
fixture_test!(links, "links");
fixture_test!(lists, "lists");
fixture_test!(math_blocks, "math-blocks");
fixture_test!(misc_blocks, "misc-blocks");
fixture_test!(paragraphs, "paragraphs");
fixture_test!(real_world, "real-world");
fixture_test!(spans_attrs, "spans-attrs");
fixture_test!(strike_highlight, "strike-highlight");
fixture_test!(super_sub, "super-sub");
fixture_test!(table_captions, "table-captions");
fixture_test!(tables, "tables");
fixture_test!(unicode_stress, "unicode-stress");
fixture_test!(wikilinks, "wikilinks");

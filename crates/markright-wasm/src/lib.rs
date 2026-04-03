use wasm_bindgen::prelude::*;

fn with_doc<T>(input: &str, f: impl FnOnce(&markright::ast::block::Document) -> T) -> T {
    let bump = markright::Bump::new();
    let doc = markright::parse(input, &bump);
    f(&doc)
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<String, JsValue> {
    with_doc(input, |doc| {
        serde_json::to_string(doc).map_err(|e| JsValue::from_str(&e.to_string()))
    })
}

#[wasm_bindgen]
pub fn parse_to_html(input: &str) -> String {
    with_doc(input, markright::to_html)
}

#[wasm_bindgen]
pub fn parse_to_html_with_options(input: &str, options: JsValue) -> Result<String, JsValue> {
    let opts: markright::HtmlOptions =
        serde_wasm_bindgen::from_value(options).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(with_doc(input, |doc| {
        markright::to_html_with_options(doc, &opts)
    }))
}

#[wasm_bindgen]
pub fn format(input: &str) -> String {
    with_doc(input, markright::to_string)
}

#[wasm_bindgen]
pub fn lint(input: &str) -> String {
    with_doc(input, |doc| {
        markright::lint(doc)
            .iter()
            .map(|l| l.message.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    })
}

#[wasm_bindgen]
pub fn schema() -> String {
    serde_json::to_string(&markright::json_schema()).unwrap()
}

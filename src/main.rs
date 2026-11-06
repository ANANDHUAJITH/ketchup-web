use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

const DESIGN: &str = include_str!("../assets/new-design.html");

fn extract_tag<'a>(source: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start = source.find(&open)?;
    let content_start = source[start..].find('>')? + start + 1;
    let end = source[content_start..].find(&close)? + content_start;
    Some(&source[content_start..end])
}

fn main() {
    let Some(win) = window() else { return };
    let Some(document) = win.document() else { return };

    // The supplied HTML design is embedded in the Rust/WASM bundle. Its styling,
    // assets and interactive controls are mounted into the page at startup.
    if let Some(style) = extract_tag(DESIGN, "style") {
        if let Ok(style_el) = document.create_element("style") {
            style_el.set_text_content(Some(style));
            if let Some(head) = document.head() {
                let _ = head.append_child(&style_el);
            }
        }
    }

    if let Some(body) = extract_tag(DESIGN, "body") {
        if let Some(body_el) = document.body() {
            body_el.set_inner_html(body);
        }
    }

    if let Some(script) = extract_tag(DESIGN, "script") {
        if let Ok(script_el) = document.create_element("script") {
            script_el.set_text_content(Some(script));
            if let Some(body_el) = document.body() {
                let _ = body_el.append_child(&script_el);
            }
        }
    }

    if let Some(root) = document.document_element() {
        let _ = root.set_attribute("data-theme", "light");
    }
}

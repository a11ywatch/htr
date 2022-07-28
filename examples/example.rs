extern crate htr;

use htr::convert_props_react;

fn main() {
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;
    let react_html = convert_props_react(&html.to_string());

    println!("{}", react_html);
}

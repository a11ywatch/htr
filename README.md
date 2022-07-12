# html-to-react

convert html props into valid react props

## Getting Started

This project takes a html string and replaces the properties with the valid react.js markup.
This helps when using find and replace tools since when you scrape content giving raw html and need a way to convert it back to the form it would be in a codebase to search.

```rust
extern crate html_to_react;

fn main(){
    let html = r#"<img class="something">"#;
    let react_html = convert_props_react(html.to_string());

    println!("{react_html}");
    // <img className="something">
}
```

This package only handles converting the `property` key to the react counter part. It does not modify the values as in converting `styles="width=300px"` to `style={{width: 300}}`.
We have this conversion handled in our [code_fix A11yWatch CLI](https://github.com/A11yWatch/a11ywatch/blob/main/cli/src/fs/code_fix.rs) and may add this in later as a feature.

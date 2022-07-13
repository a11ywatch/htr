# html-to-react

Fast html to react jsx converter

## Getting Started

This project convers html into valid react jsx markup.

This helps when using find and replace tools when you scrape content getting raw html and need a way to convert it back to the form it would be in a valid React codebase.

```toml
[dependencies]
html_to_react = "0.4.0"
```

```rust
extern crate html_to_react;

use html_to_react::convert_props_react;

fn main(){
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;
    let react_html = convert_props_react(html.to_string());

    println!("{}", react_html);
    // <div className="something" htmlFor="mystuff" tabIndex="2" style={{color: white, backgroundColor: black}}>
}
```

## About

This project uses BTrees and parses the html with the order sorted before lookup for speed.

You can combine this project with [html-parser](https://docs.rs/html_parser/latest/html_parser/) to convert elements into React ready components.

An [example](https://github.com/A11yWatch/a11ywatch/blob/main/cli/src/fs/code_fix.rs) of this being used with [ripgrep](https://github.com/BurntSushi/ripgrep) to perform search and replace from scraped content to find exactly where in the codebase it would be, the react way.

## TODO

1. ~~Allow elements with child elements to convert ex: `<ul><li></li></ul>`~~.
1. Allow dumping fully html to fragment components automatically.

## License

[MIT](./LICENSE)

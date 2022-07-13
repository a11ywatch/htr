# htr

Fast html to react jsx transformer

## Getting Started

This project transforms html into react jsx markup.

This helps when using find and replace tools when you scrape content getting raw html and need a way to convert it back to the form it would be in a valid React codebase.

```toml
[dependencies]
htr = "0.5.4"
```

```rust
extern crate htr;
use htr::convert_props_react;

fn main(){
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;
    let react_html = convert_props_react(html.to_string());

    println!("{}", react_html);
    // <div className="something" htmlFor="mystuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
}
```

```rust
extern crate htr;
use htr::convert_to_react;

/// convert html to react component
fn main(){
    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">"#;
    let react_component = convert_to_react(html.to_string(), "Something");

    println!("{}", react_component);
    // import React from "react"
    //
    // function Something() {
    //     return (
    //         <div className="something" htmlFor="mystuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
    //             <div className="child" htmlFor="mychildstuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
    //                 child
    //             </div>
    //         </div>
    //     )
    // }
}
```

## Benchmark Results

### transform

| `64gb apple silicon`                                  | `parse`                     |
| :---------------------------------------------------- | :-------------------------- |
| **`convert_props_react: simultaneous 1000 samples`**  | `4.86 us` (✅ **1.00x**)    |
| **`convert_props_react: concurrent10x 1000 samples`** | `45.130 us` (✅ **10.00x**) |

## About

This project uses BTrees and parses the html with the order sorted before lookup for speed.

You can combine this project with [html-parser](https://docs.rs/html_parser/latest/html_parser/) to convert elements into React ready components.

An [example](https://github.com/A11yWatch/a11ywatch/blob/main/cli/src/fs/code_fix.rs) of this being used with [ripgrep](https://github.com/BurntSushi/ripgrep) to perform search and replace from scraped content to find exactly where in the codebase it would be, the react way.

## License

[MIT](./LICENSE)

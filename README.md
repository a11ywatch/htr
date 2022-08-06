# htr

Fast html to react jsx transformer

## Getting Started

This project transforms html into react jsx markup.

This helps when using find and replace tools when you scrape content getting raw html and need a way to convert it back to the form it would be in a valid React codebase.

```toml
[dependencies]
htr = "0.5.26"
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
    let html = r#"
        <script>window.alert()</script>
        <div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">
            <div class="child" for="mychildstuff" tabindex="2">
                child
            </div>
        </div>
        <span class="react" style="position: relative; display: block; margin-left: auto; margin-right: auto; max-width: 577px; "
        ></span>
        "#;

    let react_component = convert_to_react(&html.to_string(), "Something");

    println!("{}", react_component);
    // import React from "react"
    //
    // function Something() {
    //     return (
    //        <>
    //            <script>{`window.alert()`}</script>
    //                <div className="something" htmlFor="mystuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
    //                  <div className="child" htmlFor="mychildstuff" tabIndex="2">
    //                      child
    //                  </div>
    //              </div>
    //              <span className="react" style={{position: "relative", display: "block", marginLeft: "auto", marginRight: "auto", maxWidth: "577px"}}
    //              ></span>
    //        </>
    //     )
    // }
}
```

## CLI

If you need [Command Line](./htr_cli/) usage.

```
cargo install htr_cli

# transform to react
htr transform --html '<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">'
```

## Benchmark Results

### transform

| `64gb apple silicon`                                  | `parse`                     |
| :---------------------------------------------------- | :-------------------------- |
| **`convert_props_react: simultaneous 1000 samples`**  | `4.86 us` (✅ **1.00x**)    |
| **`convert_props_react: concurrent10x 1000 samples`** | `45.130 us` (✅ **10.00x**) |

## About

This project uses BTrees and parses the html with the order sorted before lookup for speed.

An [example](https://github.com/A11yWatch/a11ywatch/blob/main/cli/src/fs/code_fix.rs) of this being used with [ripgrep](https://github.com/BurntSushi/ripgrep) to perform search and replace from scraped content to find exactly where in the codebase it would be, the react way.

## TODO

1. inline symbols handling ex: `<div>{</div>`

## License

[MIT](./LICENSE)

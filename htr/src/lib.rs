#![warn(missing_docs)]

//! HTML to react transformer
//!
//! htr is a fast html to react
//! transformer that uses btrees to search and replace
//! html properties to the react equivalent.
//!
//! # How to use htr
//!
//! There are two ways to use htr:
//!
//! - **Convert props React** transform html string to a react string.
//!   - [`convert_props_react`] is used to transform :blocking.
//! - **Convert to React**  Lets you transform the html to a react component.
//!   - [`convert_to_react`] is used to transform :blocking.
//!
//! [`convert_props_react`]: htr/fn.convert_props_react.html
//! [`convert_to_react`]: htr/fn.convert_to_react.html
//!
//! # Basic usage
//!
//! First, you will need to add `htr` to your `Cargo.toml`.
//!
//! Next, add your html to one of the transform methods to get your react,
//! output.

#[macro_use]
extern crate lazy_static;

extern crate convert_case;

/// transform functions
mod transform;
/// application utils
mod utils;
pub use transform::{create_style_object, extract_html_props};
/// static map of html props
mod statics;
pub use statics::{HTML_PROPS, SELF_ENCLOSED_TAGS};

/// convert props to react
pub fn convert_props_react(ctx: &str) -> String {
    let mut context = ctx.to_string();
    let props: Vec<String> = extract_html_props(&context);

    for item in props.iter() {
        if item == "style" {
            context = create_style_object(&context);
        } else {
            let value = HTML_PROPS.get(&*item.to_owned()).unwrap_or(&"");

            if !value.is_empty() {
                context = context.replace(&format!("{}=", item), &format!("{}=", value));
            }
        }
    }

    context
}

/// convert props to a react component
pub fn convert_to_react(ctx: &String, component_name: String) -> String {
    let react_html = convert_props_react(ctx);
    let mut react_html = react_html.trim().to_owned();

    // remove html tags
    if react_html.starts_with("<!DOCTYPE html>") {
        react_html = react_html.replace("<!DOCTYPE html>", "").trim().to_owned();
    }
    if react_html.starts_with("<html>") {
        react_html = react_html.replace("<html>", "");
        react_html = react_html.replace("</html>", "");
    }
    // add slow re-iterate contains [TODO get values upfront in convert_props]
    if react_html.contains("<script")
        || react_html.contains("<style")
        || react_html.contains("<script>")
        || react_html.contains("<style>")
    {
        react_html = convert_children_react(&mut react_html);
    }

    let component_name = format!(" {}", component_name.trim());

    let component = format!(
        r###"import React from "react"

function{}() {{
    return (
        <>
        {}
        </>
    )
}}"###,
        component_name, react_html
    );

    component
}

/// transform inline react children from script and style tags
pub fn convert_children_react(ctx: &mut String) -> String {
    let mut entry_start = false; // entry start
    let mut entry_end = false; // end of tag
    let mut inside_tag = false; // inside a start tag
    let mut store_tag = false; // can store tagname
    let mut current_prop = String::from(""); // current tagname

    let mut result = String::from("");
    let mut peekable = ctx.chars().peekable();

    let mut empty_children = false; // prevent updating empty children

    // TODO: capture url(base64; comma cases to build next
    let mut block_self_enclose = false;

    while let Some(c) = peekable.next() {
        result.push(c);

        // peek into next to prevent sets
        let peeked = if c == '/' || entry_start || entry_end {
            if let Some(cc) = peekable.peek() {
                cc.to_string()
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };

        if c == '<' {
            inside_tag = true;
            store_tag = true;
        }
        if c == '/' && peeked == ">" {
            block_self_enclose = true;
        }
        if c == '>' {
            inside_tag = false;
            // self enclose the tag
            if SELF_ENCLOSED_TAGS.contains(&current_prop.trim_end().as_ref()) {
                if !block_self_enclose {
                    result.pop();
                    result.push('/');
                    result.push('>');
                } else {
                    block_self_enclose = false;
                }
            }
        }

        // entry start children append [TODO: all tags that require inlining]
        if entry_start && c == '>' {
            entry_start = false;
            store_tag = true;

            if peeked != "<" {
                result.push('{');
                result.push('`');
                empty_children = false;
            } else {
                empty_children = true;
            }

            current_prop.clear();
        }

        // entry end children prepend
        if entry_end {
            if !empty_children {
                for _ in 0..current_prop.len() + 1 {
                    result.pop();
                }
                result.push('`');
                result.push('}');
                result.push_str(&format!(r#"{}>"#, current_prop));
            } else {
                empty_children = true;
            }
            entry_end = false;

            current_prop.clear();
        }

        if inside_tag && store_tag {
            current_prop.push(c);

            // start of tag
            if current_prop == "<style" || current_prop == "<script" {
                entry_start = true;
                empty_children = false; // reset empty children detection
            }

            // end of tag
            if current_prop == "</style" || current_prop == "</script" {
                entry_end = !empty_children;
            }

            // finish storing tags
            if c == ' ' {
                store_tag = false;
            }

            // end tag prevent store
            if current_prop.starts_with("</") && c == '>' {
                store_tag = false;
            }
        } else if !inside_tag {
            current_prop.clear();
        }
    }

    result
}

use crate::utils::text_between;
use convert_case::{Case, Casing};

/// extract all html props into a vector
pub fn extract_html_props(context: &String) -> Vec<String> {
    let mut props: Vec<String> = vec![];
    let mut current_prop = String::from("");
    let mut space_before_text = false;
    let mut inside_tag = false;

    // get all html props into a vec
    for c in context.chars() {
        if inside_tag {
            if c == '=' {
                space_before_text = false;
                props.push((*current_prop.trim()).to_string());
                current_prop.clear();
            }
            if space_before_text {
                current_prop.push(c);
            }
            if c == ' ' {
                space_before_text = true;
            }
        }
        if c == '<' {
            inside_tag = true;
        }
        if c == '>' {
            inside_tag = false;
        }
    }

    // sort the vec for btree linear lookup performance
    props.sort();

    props
}

/// manipulate and create the style properties to react
pub fn create_style_object(cc: &String) -> String {
    let style_matcher = if cc.contains("style='") {
        r#"'"#
    } else {
        r#"""#
    };

    let mut ctx = cc.clone();
    let style_start = format!(r#"style={}"#, style_matcher);
    let (style_string, start_idx, end_idx) = text_between(&ctx, &style_start, style_matcher);
    let mut current_prop = String::from("");
    let mut style_replacer = style_string.clone();

    // determine if base64 img url
    let mut base64_value = false;

    // replace inline property names and semi-colons to commas
    for c in style_string.chars() {
        current_prop.push(c);

        if c == ';' {
            style_replacer = style_replacer.replacen(";", ",", 1);
            current_prop.clear();
        }
        if c == ':' {
            // track base64 includes for fast re-replace
            if current_prop == " url('data:" || current_prop == "url('data:" {
                base64_value = true;
            }
            let clp = current_prop.trim();
            style_replacer = style_replacer.replacen(&clp, &clp.to_case(Case::Camel), 1);
            current_prop.clear();
        }
    }

    // contains base64 value needs to re-replace data
    if base64_value {
        style_replacer = style_replacer.replace(",base64,", ";base64,")
    }

    // add property value quotes
    let mut space_before_text = false;
    let mut needs_insert_quote = false; // add quotes after first :
    let mut style_string = String::from("");

    let mut current_value = String::from("");

    // add double quotes to react props style values
    for (i, c) in style_replacer.chars().enumerate() {
        current_value.push(c);

        // insert at non empty whitespace beforehand
        if c != ' ' && space_before_text && needs_insert_quote {
            style_string.push('\"');
            needs_insert_quote = false;
        }

        style_string.push(c);

        if !space_before_text && c == ':' {
            space_before_text = true;
            needs_insert_quote = true;
            current_value.clear();
        }

        if space_before_text && c == ',' || space_before_text && i + 1 == style_replacer.len() {
            if current_value.contains(";base64,") {
                // clear out tracker
                current_value.clear();
            } else {
                if c == ',' {
                    style_string.pop();
                    style_string.push('\"');
                    style_string.push(',');
                } else {
                    style_string.push('\"');
                }
                space_before_text = false;
            }
        }
    }

    // clean styles for any trailing commas
    style_string = style_string.trim_end().to_string();
    if style_string.ends_with(",") {
        style_string.pop();
    }

    let style_replacer = format!("{}{}{}", "style={{", style_string, "}}");

    ctx.replace_range(start_idx - 7..start_idx + end_idx + 1, &style_replacer);

    ctx
}

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

    // replace inline propert names and semi-colons to commas
    for c in style_string.chars() {
        current_prop.push(c);

        if c == ';' {
            style_replacer = style_replacer.replace(";", ",");
            current_prop.clear();
        }
        if c == ':' {
            let clp = current_prop.trim();
            style_replacer = style_replacer.replace(&clp, &clp.to_case(Case::Camel));
            current_prop.clear();
        }
    }

    // add property value quotes
    let mut space_before_text = false;
    let mut needs_insert_quote = false; // add quotes after first :
    let mut style_string = String::from("");

    // add double quotes to react props style values
    for (i, c) in style_replacer.chars().enumerate() {
        // insert at non empty whitespace beforehand
        if c != ' ' && space_before_text && needs_insert_quote {
            style_string.push('\"');
            needs_insert_quote = false;
        }

        style_string.push(c);

        if !space_before_text && c == ':' {
            space_before_text = true;
            needs_insert_quote = true;
        }

        if space_before_text && c == ',' || space_before_text && i + 1 == style_replacer.len() {
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
    
    // clean styles for any trailing commas
    style_string = style_string.trim_end().to_string();
    if style_string.ends_with(",") {
        style_string.pop();
    }

    let style_replacer = format!("{}{}{}", "style={{", style_string, "}}");

    ctx.replace_range(start_idx - 7..start_idx + end_idx + 1, &style_replacer);

    ctx
}

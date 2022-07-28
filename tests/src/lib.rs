#[test]
fn convert_props_react_test() {
    use htr::convert_props_react;

    // convert special props class
    let html = r#"<img class="something">"#;
    let react_html = convert_props_react(html.to_string());

    assert_eq!("<img className=\"something\">", react_html);

    // convert special props class and for
    let html = r#"<img class="something" for="mystuff">"#;
    let react_html = convert_props_react(html.to_string());

    assert_eq!(
        "<img className=\"something\" htmlFor=\"mystuff\">",
        react_html
    );

    // convert special props class, for, and other props
    let html = r#"<img class="something" for="mystuff" tabindex="2">"#;
    let react_html = convert_props_react(html.to_string());

    assert_eq!(
        "<img className=\"something\" htmlFor=\"mystuff\" tabIndex=\"2\">",
        react_html
    );
}

#[test]
fn extract_html_props_test() {
    use htr::extract_html_props;

    let html = r#"<img class="something" for="mystuff" tabindex="2">"#;
    let props = extract_html_props(&html.to_string());

    assert_eq!(props, vec!["class", "for", "tabindex"]);
}

#[test]
fn convert_props_react_styles_test() {
    use htr::convert_props_react;

    let html = r#"<img style="color: white; background-color: black">"#;
    let props = convert_props_react(html.to_string());

    assert_eq!(
        r#"<img style={{color: "white", backgroundColor: "black"}}>"#,
        props
    );
}

#[test]
fn convert_props_react_children_test() {
    use htr::convert_props_react;

    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">
    <div class="child" for="mychildstuff" tabindex="2" style="color: white; background-color: black">
        child
    </div>
</div>"#;
    let props = convert_props_react(html.to_string());

    assert_eq!(
        r###"<div className="something" htmlFor="mystuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
    <div className="child" htmlFor="mychildstuff" tabIndex="2" style={{color: "white", backgroundColor: "black"}}>
        child
    </div>
</div>"###,
        props
    );
}

#[test]
fn convert_react_component_test() {
    use htr::convert_to_react;

    let html = r#"<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">
            <div class="child" for="mychildstuff" tabindex="2" style="color: white; background-color: black">
                child
            </div>
        </div>"#;

    let props = convert_to_react(html.trim().to_string(), "Something".to_string());

    let style_object = r#"style={{color: "white", backgroundColor: "black"}}"#;

    assert_eq!(
        format!(
            r###"import React from "react"
    
function Something() {{
    return (
        <>
        <div className="something" htmlFor="mystuff" tabIndex="2" {}>
            <div className="child" htmlFor="mychildstuff" tabIndex="2" {}>
                child
            </div>
        </div>
        </>
    )
}}"###,
            style_object, style_object
        ),
        props
    );
}

#[test]
fn convert_props_react_test() {
    use htr::convert_props_react;

    // convert special props class
    let html = r#"<img class="something">"#;
    let react_html = convert_props_react(&html.to_string());

    assert_eq!("<img className=\"something\">", react_html);

    // convert special props class and for
    let html = r#"<img class="something" for="mystuff">"#;
    let react_html = convert_props_react(&html.to_string());

    assert_eq!(
        "<img className=\"something\" htmlFor=\"mystuff\">",
        react_html
    );

    // convert special props class, for, and other props
    let html = r#"<img class="something" for="mystuff" tabindex="2">"#;
    let react_html = convert_props_react(&html.to_string());

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
    let props = convert_props_react(&html.to_string());

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
    let props = convert_props_react(&html.to_string());

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

    let props = convert_to_react(&html.trim().to_string(), "Something".to_string());

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


/// basic react component to children test
#[test]
fn convert_react_component_children_test() {
    use htr::convert_to_react;
    let html = r#"
        <script>window.alert()</script>
        <div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">
            <div class="child" for="mychildstuff" tabindex="2">
                child
            </div>
        </div>
        <script>window.alert()</script>
        <img src="img_girl.jpg" alt="Girl in a jacket" width="500" height="600">
        <span class="react" style="position: relative; display: block; margin-left: auto; margin-right: auto; max-width: 577px; "
        ></span>
        "#;

    let props = convert_to_react(&html.trim().to_string(), "Something".to_string());

    let style_object = r#"style={{color: "white", backgroundColor: "black"}}"#;
    let style_object2 = r#"style={{position: "relative", display: "block", marginLeft: "auto", marginRight: "auto", maxWidth: "577px"}}"#;

    // uncomment to debug
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("foo.ts").unwrap();
    file.write_all(props.as_bytes()).unwrap();

    let results = format!(
        r#"import React from "react"

function Something() {{
    return (
        <>
        <script>{{`window.alert()`}}</script>
        <div className="something" htmlFor="mystuff" tabIndex="2" {}>
            <div className="child" htmlFor="mychildstuff" tabIndex="2">
                child
            </div>
        </div>
        <script>{{`window.alert()`}}</script>
        <img src="img_girl.jpg" alt="Girl in a jacket" width="500" height="600"/>
        <span className="react" {}
        ></span>
        </>
    )
}}"#,
        style_object,
        style_object2
    );

    assert_eq!(
        results,
        props
    );
}

#[test]
fn convert_react_component_children_test_advanced() {
    use htr::convert_to_react;
    let html = r#"
        <script>window.alert()</script>
        <div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">
            <div class="child" for="mychildstuff" tabindex="2">
                child
            </div>
        </div>
        <script>window.alert()</script>
        <span class="react" style="position:absolute;top:0;left:0;opacity:1;width:100%;height:100%;object-fit:cover;object-position:center"
        ></span>
        "#;

    let props = convert_to_react(&html.trim().to_string(), "Something".to_string());

    let style_object = r#"style={{color: "white", backgroundColor: "black"}}"#;
    let style_object2 = r#"style={{position:"absolute",top:"0",left:"0",opacity:"1",width:"100%",height:"100%",objectFit:"cover",objectPosition:"center"}}"#;
    
    // use std::fs::File;
    // use std::io::prelude::*;

    // let mut file = File::create("foo.tsx").unwrap();
    // file.write_all(props.as_bytes()).unwrap();

    let results = format!(
        r#"import React from "react"

function Something() {{
    return (
        <>
        <script>{{`window.alert()`}}</script>
        <div className="something" htmlFor="mystuff" tabIndex="2" {}>
            <div className="child" htmlFor="mychildstuff" tabIndex="2">
                child
            </div>
        </div>
        <script>{{`window.alert()`}}</script>
        <span className="react" {}
        ></span>
        </>
    )
}}"#,
        style_object,
        style_object2
    );

    assert_eq!(
        results,
        props
    );
}
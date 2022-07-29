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
        <div style="background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAaCAYAAAC3g3x9AAAACXBIWXMAAAsTAAALEwEAmpwYAAAD9UlEQVRIx51US28bVRT2X2GFhFgi8QtYIcQGJBBBVeUgJazYlF1DkkX5A2WFREtpyoICieMkhbQJ3ZAunLR1nObpx/gx4/FMPO/3jL/qnLEnTpqw4EqfvnPvnPvd87hzc7dv/4C1R39haamAwnIRhUIRy8UVFIurKK6k4PnK2Zy+jXyJFxcLvH7jxrfIbWw+RRgPoBsWTMthqKc65N4purKCrqxCUTVmgtTtQe6pmb/teGh3JF67det75FZW19DXDLQ7IjpiF6Ik4/CoivLuK+w8L2O3so+9vQNs77zEi5cVlEovmAWhzb6EZrMN9VTDzMx3qaCmmyxGp5MDRUYnyoqack/J7N6QR75SV+a9hmnj5s0Z5NbXnyAIIvT7OnTdzKAxjCGP7DOM+9LeMIwxOzuH3ObmU9CwLAeO7cFmuOfgOB4cxx/yRfi8l8b8/DwJ/oPBADBNG74fIghCRFGcgaJ3XTrEeeOgFA7vpTE3LmgYJgRBQKvVwuHhIcrlMnOtVoNtpxsGg8E50IjjOBO8EKEFTdMgyzKLNBoCqtUaOh0Rvu+zQBwnjCRJOYoThGF0uaBl2ZxemnLCRU6ZUh7VL60npeq7LgahB9dxr45QURRYlgVdN5ipDLROh5mmyfWiNd9zUe85+PuVhp7uwrFsDC4KkmO1WkW9XsezZ1s4OjrCzs4OKpUK9vf3USqVcHJyjK2tf9FstqB2G9h4vAy9r3LUpHFOkE6n2hBGRU+SARc9iiIGfSPnhOBbgN1F6FPKF64NDUqTIhBFEY1GAwcHB2i321l3qVmqqmJ7exuGYWA04iS5uime58N1Xa5fWjObIwuCgNc9z4Ou68xRGCEMQwR+cFVTbL7xJEoXPEXAHSZ4XsCgdeoy+Y5wqSD9k3QtRLGLTkeCJPXYbrVESJI8XJMZ438KZWYY1uWC9Is1mx1Uqw2cnNRxfFxj0FNF83q9iUajld1FYorwUkHDsKGbNj+YtuvDdnxmZ8xmdnwYpnMOb6ScDADPdoDAB3wvRfBf8M+B0s4ENzbSa7MnGvhlt49f9zQ82Ouzfafcx73d1B7xz+V+5rdQ6eP3fQ2qPnxt5uaRe/wkFbz7XMFbPwp4924T791vIf9IwvU1CZ8sifi0IGJ6vYtrqxK+2ZDx2bKIt38S8M6dJt6/34KgmKwxmwpu8mRxV8EH92r4+EGD8cVDAdOFFiYeCmxf+6PJfP3PJj7/TcBHCw18uFBnu6OaZxFyDRNwpzRqjGEzq5qNXj/lU92GotmcmjKcj/x0I+0yNTYTpOF7ASJ6ssYQR2Pz6JK1MOZnji58FmE+n8fU1BTy+UnkJ/8H8pOYnPwK09NfY2LiS7wG6DBFsQuOS6oAAAAASUVORK5CYII=');"></div>
        <span class="react" style="position: relative; display: block; margin-left: auto; margin-right: auto; max-width: 577px; "
        ></span>
        "#;

    let props = convert_to_react(&html.trim().to_string(), "Something".to_string());

    let style_object = r#"style={{color: "white", backgroundColor: "black"}}"#;
    let style_object2 = r#"style={{position: "relative", display: "block", marginLeft: "auto", marginRight: "auto", maxWidth: "577px"}}"#;
    let style_object3 = r#"style={{backgroundImage: "url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAaCAYAAAC3g3x9AAAACXBIWXMAAAsTAAALEwEAmpwYAAAD9UlEQVRIx51US28bVRT2X2GFhFgi8QtYIcQGJBBBVeUgJazYlF1DkkX5A2WFREtpyoICieMkhbQJ3ZAunLR1nObpx/gx4/FMPO/3jL/qnLEnTpqw4EqfvnPvnPvd87hzc7dv/4C1R39haamAwnIRhUIRy8UVFIurKK6k4PnK2Zy+jXyJFxcLvH7jxrfIbWw+RRgPoBsWTMthqKc65N4purKCrqxCUTVmgtTtQe6pmb/teGh3JF67det75FZW19DXDLQ7IjpiF6Ik4/CoivLuK+w8L2O3so+9vQNs77zEi5cVlEovmAWhzb6EZrMN9VTDzMx3qaCmmyxGp5MDRUYnyoqack/J7N6QR75SV+a9hmnj5s0Z5NbXnyAIIvT7OnTdzKAxjCGP7DOM+9LeMIwxOzuH3ObmU9CwLAeO7cFmuOfgOB4cxx/yRfi8l8b8/DwJ/oPBADBNG74fIghCRFGcgaJ3XTrEeeOgFA7vpTE3LmgYJgRBQKvVwuHhIcrlMnOtVoNtpxsGg8E50IjjOBO8EKEFTdMgyzKLNBoCqtUaOh0Rvu+zQBwnjCRJOYoThGF0uaBl2ZxemnLCRU6ZUh7VL60npeq7LgahB9dxr45QURRYlgVdN5ipDLROh5mmyfWiNd9zUe85+PuVhp7uwrFsDC4KkmO1WkW9XsezZ1s4OjrCzs4OKpUK9vf3USqVcHJyjK2tf9FstqB2G9h4vAy9r3LUpHFOkE6n2hBGRU+SARc9iiIGfSPnhOBbgN1F6FPKF64NDUqTIhBFEY1GAwcHB2i321l3qVmqqmJ7exuGYWA04iS5uime58N1Xa5fWjObIwuCgNc9z4Ou68xRGCEMQwR+cFVTbL7xJEoXPEXAHSZ4XsCgdeoy+Y5wqSD9k3QtRLGLTkeCJPXYbrVESJI8XJMZ438KZWYY1uWC9Is1mx1Uqw2cnNRxfFxj0FNF83q9iUajld1FYorwUkHDsKGbNj+YtuvDdnxmZ8xmdnwYpnMOb6ScDADPdoDAB3wvRfBf8M+B0s4ENzbSa7MnGvhlt49f9zQ82Ouzfafcx73d1B7xz+V+5rdQ6eP3fQ2qPnxt5uaRe/wkFbz7XMFbPwp4924T791vIf9IwvU1CZ8sifi0IGJ6vYtrqxK+2ZDx2bKIt38S8M6dJt6/34KgmKwxmwpu8mRxV8EH92r4+EGD8cVDAdOFFiYeCmxf+6PJfP3PJj7/TcBHCw18uFBnu6OaZxFyDRNwpzRqjGEzq5qNXj/lU92GotmcmjKcj/x0I+0yNTYTpOF7ASJ6ssYQR2Pz6JK1MOZnji58FmE+n8fU1BTy+UnkJ/8H8pOYnPwK09NfY2LiS7wG6DBFsQuOS6oAAAAASUVORK5CYII=')"}}"#;

    // uncomment to debug
    // use std::fs::File;
    // use std::io::prelude::*;

    // let mut file = File::create("foo.ts").unwrap();
    // file.write_all(props.as_bytes()).unwrap();

    let results = format!(
        r#"import React from "react"

function Something() {{
    return (
        <>
        <script>{{`window.alert()`}}</script>
        <div className="something" htmlFor="mystuff" tabIndex="2" {style_object}>
            <div className="child" htmlFor="mychildstuff" tabIndex="2">
                child
            </div>
        </div>
        <script>{{`window.alert()`}}</script>
        <img src="img_girl.jpg" alt="Girl in a jacket" width="500" height="600"/>
        <div {style_object3}></div>
        <span className="react" {style_object2}
        ></span>
        </>
    )
}}"#);

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
extern crate htr;

use htr::convert_to_react;

fn main() {
    let html = r#"<!DOCTYPE html>
    <html>
      <head>
        <title>HTR</title>
        <style>
          .h1 {
            color: #ccc;
          }
        </style>
      </head>
    
      <body>
        <h1>My Web Page</h1>
    
        <p>Welcome to HTR!</p>
    
        <p>content goes a long way.&hellip;</p>
    
        <p>We could put more here.</p>
        <address>
          Made 15 July 2022<br />
          by myself.
        </address>
      </body>
    </html>"#;
    let react_html = convert_to_react(&html.to_string(), "MyPage".to_string());

    println!("{}", react_html);
}

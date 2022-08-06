extern crate htr;

pub mod options;

use clap::Parser;
use options::{Cli, Commands};
use htr::{convert_props_react, convert_to_react};
use std::io::{self, Write};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::TRANSFORM { html, component_name }) => {
            let component_name: String = component_name.to_owned().unwrap_or_default();

            if component_name.is_empty() {
                let react_html = convert_props_react(&html.to_string());

                io::stdout().write_all(react_html.as_bytes()).unwrap();
            } else {
                let react_html = convert_to_react(&html.to_string(), component_name);

                io::stdout().write_all(react_html.as_bytes()).unwrap();
            }
        }
        None => {}
    }
}

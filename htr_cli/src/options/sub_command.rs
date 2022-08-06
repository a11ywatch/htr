use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// transform HTML to a react.
    TRANSFORM {
        /// the html to manipulate
        #[clap(long)]
        html: String,
        /// transform to a react component
        #[clap(short, long)]
        component_name: Option<String>,
    },
}

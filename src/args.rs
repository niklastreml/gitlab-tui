use clap::Parser;

use crate::app::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("origin"))]
    pub remote: String,
}

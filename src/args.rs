use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Set this to your remote's name, if you're using a different name than 'origin'
    #[arg(short, long, default_value_t = String::from("origin"))]
    pub remote: String,
}

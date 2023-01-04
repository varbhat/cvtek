use clap::Parser;

// Flag Struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub from: Option<String>,

    #[arg(short, long)]
    pub to: Option<String>,

    #[arg(short, long)]
    pub out: bool,

    // #[arg(long="in")]
    // pub stdin: bool,
    #[arg(long)]
    pub template: Option<String>,

    #[arg(long = "type")]
    pub output_type: Option<String>,
}

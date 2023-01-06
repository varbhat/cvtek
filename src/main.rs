mod cvstruct;
mod flags;

use clap::Parser;

use crate::cvstruct::CVStruct;

use cvstruct::OutputType;

use std::io::{self, Read};

fn main() -> anyhow::Result<()> {
    let args = flags::Args::parse();

    let cv = if args.stdin {
        let mut stdinput = String::new();
        io::stdin().read_to_string(&mut stdinput)?;
        CVStruct::from_toml(&stdinput)?
    } else {
        match args.from {
            Some(filepath) => CVStruct::from_toml_file(&filepath)?,
            None => CVStruct::get_dummy(),
        }
    };

    let out_type = match args.to.to_owned() {
        Some(filepath) => {
            if filepath.ends_with(".tex") {
                OutputType::Tex
            } else if filepath.ends_with(".md") {
                OutputType::Markdown
            } else if filepath.ends_with(".html") {
                OutputType::HTML
            } else if filepath.ends_with(".toml") {
                OutputType::TOML
            } else {
                OutputType::Tex
            }
        }
        None => OutputType::Tex,
    };

    let contents = match out_type {
        OutputType::Tex => cv.to_tex()?,
        OutputType::HTML => cv.to_html()?,
        OutputType::Markdown => cv.to_md()?,
        OutputType::TOML => cv.to_toml()?,
    };

    if args.out {
        print!("{contents}");
    } else {
        if let Some(filepath) = args.to {
            std::fs::write(filepath, contents)?;
        } else {
            print!("{contents}");
        }
    }
    anyhow::Ok(())
}

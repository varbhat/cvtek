mod cvstruct;
mod flags;

use clap::Parser;

use crate::cvstruct::CVStruct;

use cvstruct::OutputType;

fn main() -> anyhow::Result<()> {
    let args = flags::Args::parse();

    let cv = match args.from {
        Some(filepath) => CVStruct::from_toml_file(&filepath)?,
        None => CVStruct::get_dummy(),
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

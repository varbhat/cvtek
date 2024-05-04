# *cvtek*
## 📄 Craft your Resume/CV using TOML

![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust)
![GitHub License](https://img.shields.io/github/license/varbhat/cvtek?logoColor=violet)

cvtek is acronym for **CV** **TE**x **K**rafter.

Using cvtek, you can craft your [Résumé/CV](https://en.wikipedia.org/wiki/R%C3%A9sum%C3%A9) using a [TOML](https://toml.io) File. You feed the cvtek with a human readable/writable Resume TOML file and cvtek will generate ATS Friendly Latex Resume Source. You can then use Latex compiler (or [Latex to PDF](https://github.com/xu-cheng/latex-action)) Or [Overleaf](https://www.overleaf.com/) to compile your resume to PDF.

Example `resume.toml` will look like this:

```toml
[header]
full_name = "John Doe"
email_addr = "john@doe.com"
github_username = "github"
linkedin_username = "linkedin"
location = "Some Place, Earth"
phone_number = "1234567890"

[[education]]
course_name = "Bachelor of Technology in Computer Science"
timeline = "August 2003 - July 2007"
university_name = "University at My Place"
university_link = "https://myopenuniversity.university"
location = "Another Place, Earth"
course_grade = "A"
points = ["Did some useful work Y", "Did some useful work Z"]

[[experience]]
comp_name = "Company ABC"
comp_link = "https://mygoodcompany.company"
exp_name = "Engineer"
timeline = "August 2008 - Present"
location = "Another place, Earth"
points = ["I joined this when i had mood to join", "I joined this to prove myself", "I achieved ABC here"]

[[projects]]
title = "My good project XYZ"
timeline = "January 2006 - Present"
project_link = "https://github.com/github"
description = "Some incredible project that I created"
points = ["I created this when i had mood to create", "I created this to prove myself"]

[skills]
Technologies = "A, B, C, D, E, F, G, H, whatever"
Frameworks = "ABC, DEF, GHI, JKL, whatever"

```

Resume Produced by `cvtek` will look like this (See `cvtek` generated [`democv.pdf` here](assets/democv.pdf)):

<p align="center">
<img src="https://raw.githubusercontent.com/varbhat/cvtek/main/assets/democv.png" alt="cvtek demo resume" width=400 height=550 />
</p>
<hr>

## Usage
- cvtek can be installed using [cargo](https://github.com/rust-lang/cargo):
  ```bash
  cargo install --git https://github.com/varbhat/cvtek
  ```
- Type `cvtek -t resume.toml` to generate dummy `resume.toml`
- Modify `resume.toml` to your likening
- Compile your `resume.toml` using:
```bash
cvtek -f ./resume.toml -t resume.tex
# If you want to print latex output, type the following
# cvtek -f ./resume.toml -t /dev/stdout
# If you want to output Markdown or HTML, Write to File with suitable extension
# cvtek -f ./resume.toml -t resume.md # resume.html
```

- Use local or online Latex compiler(Ex: [Overleaf](https://www.overleaf.com/)) to compile your resume.tex to PDF

## Features
- Simple
- ATS Friendly
- Latex knowledge not required
- Write your Resume using toml
- Output to Latex or HTML or Markdown

## License
[GPL-v3](LICENSE)

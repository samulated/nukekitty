// Script to convert markdown to html
use std::env;
use std::fs;

fn main () {
    println!("Start convert!");

    // Config file


    // CLI Arguments

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Converting file {file_path}");
    // Load the Markdown file
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    // Parsing
    let mut parsed = String::from("");
    let mut frontmatter_iterator = 0;

    content.lines().for_each(|x|
        if x.starts_with("---") {
            //TODO: Differentiate frontmatter and horizontal rule line
            frontmatter_iterator += 1;
            
            if frontmatter_iterator == 2 {
                //TODO: Wrap header and start body at end of frontmatter
            }
        }
        else if frontmatter_iterator == 1 {
            //TODO: parsing front matter
        }
        else if x.starts_with('#') {
            // Header tag
            let mut h_level = 0;
            let mut y = x.chars();
            while y.next().unwrap() == '#' {
                h_level += 1;
            };
            
            let block = y.as_str();

            let s = format!("<h{h_level}>{block}</h{h_level}>\n");
            parsed.push_str(&s);
        }
        else if x.starts_with('!') {
            // Image tag
            let alt_start: usize = x.find('[').unwrap() + 1;
            let alt_end: usize = x[alt_start..].find(']').unwrap();
            let alt_text = &x[alt_start..alt_start + alt_end];

            let src_start: usize = x.find('(').unwrap() + 1;
            let src_end: usize = x[src_start..].find(')').unwrap();
            let src_text = &x[src_start..src_start + src_end];

            let s = format!("<img src=\"{src_text}\" alt=\"{alt_text}\" />\n");
            parsed.push_str(&s);
        }
        else {
            // Everything else (paragraphs)
            if x.is_empty() {
                // Do nothing (only need to wrap actual content in <p> tags)
            }
            else
            {
                //TODO: inline formatting check (eg. italic & bold)
                let s = format!("<p>{x}</p>\n");
                parsed.push_str(&s);
            }
        }
    );

    println!("Converted:\n{parsed}");

    // Saving out the html

    println!("End convert!");
}
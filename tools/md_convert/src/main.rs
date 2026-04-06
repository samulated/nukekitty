// Script to convert markdown to html
use std::env;
use std::fs;

fn main () {
    println!("Start convert!");

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Converting file {file_path}");
    // Load the Markdown file
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    // Parsing
    let mut parsed = String::from("");
    let mut frontmatter = 0;

    content.lines().for_each(|x|
        if x.starts_with("---") {
            //TODO: parsing front matter
            frontmatter += 1;
        }
        else if frontmatter == 1 {
            // Do nothing
        }
        else if x.starts_with('#') {
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
            let s = format!("<p>{x}</p>\n");
            parsed.push_str(&s);
        }
    );

    println!("Converted:\n{parsed}");

    // Saving out the html

    println!("End convert!");
}
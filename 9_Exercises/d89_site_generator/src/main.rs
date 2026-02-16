// Day 89: Project: Static Site Generator (Markdown → HTML)
// Build a CLI tool that reads .md files from a directory and generates 
// a static website with .html files using templates. You’ll use pulldown-cmark 
// for parsing Markdown and basic templating to render full HTML pages.
// Key Concepts:
// + Use walkdir to scan folders recursively
// + Parse Markdown into HTML using pulldown-cmark
// + Write full .html pages with embedded content
// + Separate logic for Markdown → HTML and Template rendering
// You now have a basic static site generator, the foundation for a custom blog 
// engine, documentation site, or developer portfolio.
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;
 
fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
 
fn render_template(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
</head>
<body>
    {}
</body>
</html>"#, title, body)
}
 
fn main() {
    let input_dir = "content";
    let output_dir = "public";
    fs::create_dir_all(output_dir).unwrap();
 
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(path).expect("Failed to read .md file");
            let html_body = markdown_to_html(&content);
 
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let output_file = format!("{}/{}.html", output_dir, file_stem);
 
            let full_page = render_template(file_stem, &html_body);
            let mut file = File::create(&output_file).expect("Failed to create .html file");
            file.write_all(full_page.as_bytes()).unwrap();
 
            println!("Generated {}", output_file);
        }
    }
}
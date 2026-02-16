// Day 96: Project: Resume Builder (TOML to PDF via HTML)
// Build a CLI tool that reads resume data from a TOML file, renders it into an HTML 
// template, and optionally converts it into a PDF using an external tool like wkhtmltopdf. 
// You'll practice TOML parsing, template injection, and PDF generation hooks.
// Key Concepts:
// + Parse config-style files with toml + serde
// + Template HTML strings in Rust
// + Automate PDF output via shell command
// You now have a Rust-based resume builder — customizable, automatable, and easy to expand 
// with themes or sections.
use serde::Deserialize;
use std::fs;
 
#[derive(Deserialize)]
struct Resume {
    name: String,
    title: String,
    email: String,
    phone: String,
    experience: Vec<Experience>,
}
 
#[derive(Deserialize)]
struct Experience {
    company: String,
    role: String,
    year: String,
}

fn render_html(resume: &Resume) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>{0}</title></head><body>
        <h1>{0}</h1><h2>{1}</h2>
        <p>Email: {2} | Phone: {3}</p><hr><h3>Experience</h3><ul>"#,
        resume.name, resume.title, resume.email, resume.phone
    );
 
    for job in &resume.experience {
        html.push_str(&format!(
            "<li><strong>{}</strong> - {} ({})</li>",
            job.role, job.company, job.year
        ));
    }
 
    html.push_str("</ul></body></html>");
    html
}

fn main() {
    let toml = fs::read_to_string("resume.toml").expect("Failed to read TOML");
    let resume: Resume = toml::from_str(&toml).expect("TOML parse failed");
 
    let html = render_html(&resume);
    fs::write("resume.html", &html).expect("Failed to write HTML");
 
    println!("Resume HTML generated: resume.html");
 
    // Optional: generate PDF using wkhtmltopdf (must be installed)
    if std::process::Command::new("wkhtmltopdf")
        .args(["resume.html", "resume.pdf"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        println!("PDF generated: resume.pdf");
    } else {
        eprintln!("wkhtmltopdf not found or failed. HTML only.");
        eprintln!("Install: https://github.com/wkhtmltopdf/packaging/releases (macOS: .pkg)");
    }
}
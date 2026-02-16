// Day 92: Project: Browser Automation with Fantoccini (WebDriver Client in Rust)
// Description:
// Build a Rust CLI app that uses fantoccini to automate a browser (headless or visible). 
// You’ll launch a WebDriver session, visit a page, and interact with elements — ideal for 
// web scraping, testing, or automation workflows.
// Key Concepts:
// + fantoccini is a WebDriver client in Rust (like Selenium)
// + Use Locator to find page elements (by text, id, tag, etc.)
// + Supports actions: click, type, wait, screenshot, navigate
// You’ve now built a browser automation tool in Rust—great for testing, scraping, monitoring, 
// or task automation.
use fantoccini::{Client, Locator};
use tokio;
 
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    println!("🕸️ Launching browser automation...");
 
    // Connect to WebDriver (ChromeDriver or GeckoDriver)
    let mut client = Client::new("http://localhost:9515").await?;
 
    // Navigate to a page
    client.goto("https://www.rust-lang.org").await?;
 
    // Get the page title
    let title = client.title().await?;
    println!("Page Title: {}", title);
 
    // Click the "Learn" link if it exists
    if let Ok(link) = client.find(Locator::LinkText("Learn")).await {
        println!("🔗 Clicking 'Learn' link...");
        link.click().await?;
    }
 
    // Take a screenshot (optional)
    let screenshot = client.screenshot().await?;
    std::fs::write("screenshot.png", &screenshot).expect("❌ Failed to save screenshot");
    println!("📸 Screenshot saved to screenshot.png");
 
    // Close the session
    client.close().await?;
 
    Ok(())
}
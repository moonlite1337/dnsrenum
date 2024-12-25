use colored::Colorize;
use regex::Regex;

// google is the default go to search for subdomains, but not the only one.
// check out https://github.com/projectdiscovery/subfinder for more inspiration
pub fn google(domain: &String) {
    println!(
        "\n{}",
        "Get extra names via google scraping:".red().underline(),
    );
    let url = format!(
        "https://google.com/search?q=site:*.{0} -site:www.{0}",
        domain
    );
    let ua = "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; en-US; rv:1.4a) Gecko/20030401";
    let cl = reqwest::blocking::Client::builder()
        .user_agent(ua)
        .build()
        .unwrap();
    let resp = cl.get(&url).send().unwrap();
    let text = resp.text().unwrap();

    let pattern = format!(
        r"https?:\/\/([a-zA-Z0-9-]+\.)+{}\b",
        domain.replace(".", "\\.")
    );
    // Compile the regular expression
    let re = Regex::new(&pattern).unwrap();

    if let Some(captures) = re.captures(text.as_str()) {
        println!("Full match: {}", captures.get(0).map_or("", |m| m.as_str()));
        println!(
            "First capturing group: {}",
            captures.get(1).map_or("", |m| m.as_str())
        );
        println!(
            "Second capturing group: {}",
            captures.get(2).map_or("", |m| m.as_str())
        );
    }
    println!("test");
}

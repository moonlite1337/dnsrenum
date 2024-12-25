use colored::Colorize;
use regex::Regex;

// google is the default go to search for subdomains, but not the only one.
// check out https://github.com/projectdiscovery/subfinder for more inspiration
pub fn google(domain: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "Google search page:".red().underline(),);
    let url = format!(
        "https://google.com/search?q=site:*.{0} -site:www.{0}",
        domain
    );
    let ua = "Mozilla/5.0 (Macintosh; U; PPC Mac OS X Mach-O; en-US; rv:1.4a) Gecko/20030401";
    let cl = reqwest::blocking::Client::builder()
        .user_agent(ua)
        .build()?;
    let resp = cl.get(&url).send()?;
    let text = resp.text()?;

    let pattern = format!(
        r"(https?):\/\/([a-zA-Z0-9-]+\.)+{}\b",
        domain.replace(".", "\\.")
    );
    let re = Regex::new(&pattern)?;

    for cp in re.captures_iter(&text) {
        if let Some(subdomain) = cp.get(1) {
            if subdomain.is_empty() {
                continue;
            }
            println!("{}{}", subdomain.as_str(), domain);
        }
    }

    Ok(())
}

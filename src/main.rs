use rust_fzf::fzf_select;
use std::process::Command;

fn main() {
    let selection_https_prepended = get_website_to_open();

    print!("{}", selection_https_prepended);
    let _child = Command::new("open")
        .arg(selection_https_prepended)
        // .arg("https://linkedin.com")
        .spawn()
        .expect("open call failed");
}

fn get_website_to_open() -> String {
    let wasted_time_urls = vec![
        "linkedin.com".to_string(),
        "twitter.com".to_string(),
        "instagram.com".to_string(),
        "google.com".to_string(),
    ];
    let selection = fzf_select(wasted_time_urls);
    format!("https://{}", selection)
}

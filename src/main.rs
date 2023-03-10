use rust_fzf::fzf_select;
use std::process::Command;

fn main() {
    let selection_https_prepended = select_website();
    open_url(&selection_https_prepended);
}

fn open_url(url: &str) {
    let _child = Command::new("open")
        .arg(url)
        .spawn()
        .expect("open call failed");
}

fn select_website() -> String {
    let wasted_time_urls = vec![
        "linkedin.com".to_string(),
        "github.com".to_string(),
        "reddit.com".to_string(),
        "twitter.com".to_string(),
        "instagram.com".to_string(),
        "google.com".to_string(),
    ];
    let selection = fzf_select(wasted_time_urls);
    format!("https://{}", selection)
}

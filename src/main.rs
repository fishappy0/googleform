use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = client.get("https://www.rust-lang.org").send().await;
    println!("{:?}", res);
}

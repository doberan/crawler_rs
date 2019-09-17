fn main() {
    let client = crawler::Client::new("https://qiita.com".to_string());
    println!("client domain = {}", client.domain);
}
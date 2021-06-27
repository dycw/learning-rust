fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, you already know".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// we have to import both trait and type to use type
use trait_example::{Summary1, Tweet1, NewsArticle1, NewsArticle2, notify1, notify2};

mod generics;
mod trait_example;
fn main() {
    generics::generics();
    summarize_tweet();
    summarize_news_article();

    let article = NewsArticle2{
        headline: String::from("India wins world cup!"),
        location: String::from("Mumbai, India"),
        author: String::from("Virat Kohli"),
        content: String::from("India once again wins the World Cup in 2025!..."),
    };
    notify1(&article);
    notify2(&article);
}

fn summarize_tweet() {
    let tweet = Tweet1{
        username: String::from("rustofficial"),
        content: String::from("new version launched!"),
        reply: false,
        retweet: false,
    };
    println!("New Tweet: {}", tweet.summarize());
}

fn summarize_news_article() {
    let article = NewsArticle1{
        headline: String::from("India wins world cup!"),
        location: String::from("Mumbai, India"),
        author: String::from("Virat Kohli"),
        content: String::from("India once again wins the World Cup in 2025!..."),
    };
    println!("New article available! {}", article.summarize())
}
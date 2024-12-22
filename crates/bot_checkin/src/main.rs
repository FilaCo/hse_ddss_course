use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    let deps = DependencyMap::new();

    let handler = Update::filter_message().branch();
    println!("Hello, world!");
}

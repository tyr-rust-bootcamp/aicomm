use ai_sdk::*;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let adapter = OpenaiAdapter::new(api_key, "gpt-4o");
    let messages = vec![Message {
        role: Role::User,
        content: "世界上最长的河流是什么？".to_string(),
    }];
    let response = adapter.complete(&messages).await.unwrap();
    println!("response: {}", response);
}

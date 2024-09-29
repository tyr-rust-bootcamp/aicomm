use ai_sdk::{AiService, Message, OllamaAdapter, Role};

#[tokio::main]
async fn main() {
    let adapter = OllamaAdapter::default();
    let messages = vec![Message {
        role: Role::User,
        content: "世界上最长的河流是什么？".to_string(),
    }];
    let response = adapter.complete(&messages).await.unwrap();
    println!("response: {}", response);
}

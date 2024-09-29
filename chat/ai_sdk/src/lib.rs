mod adapters;

pub use adapters::*;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}


#[allow(async_fn_in_trait)]
pub trait AiService {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String>;
    // other common functions
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

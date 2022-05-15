use chrono::{DateTime, Utc};
pub struct Tweet {
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
}

impl Tweet {
    pub fn new(id: i32, message: &str, posted_at: DateTime<Utc>) -> Tweet {
        Tweet {
            id: Some(id),
            message: message.into(),
            posted_at,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn create(message: &str) -> Self {
        Self {
            id: None,
            message: message.into(),
            posted_at: Utc::now(),
        }
    }
}

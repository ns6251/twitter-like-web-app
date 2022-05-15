use crate::entities::Tweet as TweetEntity;

pub struct Tweet {
    pub id: String,
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl From<TweetEntity> for Tweet {
    fn from(tweet_entity: TweetEntity) -> Self {
        Tweet {
            id: tweet_entity.id().unwrap_or(-1).to_string(),
            name: "taro".to_owned(),
            message: tweet_entity.message,
            posted_at: tweet_entity.posted_at.format("%Y/%m/%d %H:%M").to_string(),
        }
    }
}

use crate::{entities::Tweet, repositories::Tweets, views::Home};

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
    }
}

pub async fn create_tweet(repo: &impl Tweets, message: &str) {
    let tweet = Tweet::create(message);
    repo.store(&tweet).await;
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    repo.delete(id).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::Tweet, repositories::MockTweets};
    use chrono::{TimeZone, Utc};

    fn tweet(id: i32) -> Tweet {
        Tweet::new(
            id,
            format!("message {}", id).as_str(),
            Utc.ymd(2022, 5, 14).and_hms(0, 0, 0),
        )
    }

    #[tokio::test]
    async fn test_list_tweets() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![tweet(2), tweet(1)]);

        let result = list_tweets(&tweets).await;
        assert_eq!(result.tweets.len(), 2);

        let result0 = result.tweets.get(0).unwrap();
        assert_eq!(result0.message, "message 2");
        assert_eq!(result0.posted_at, "2022/05/14 00:00");
    }

    #[tokio::test]
    async fn test_list_tweets_empty() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| Vec::with_capacity(0));

        let result = list_tweets(&tweets).await;
        assert!(result.tweets.is_empty());
    }

    #[tokio::test]
    async fn test_create_tweet() {
        let mut tweets = MockTweets::new();
        tweets
            .expect_store()
            .withf(|e| e.message == tweet(1).message)
            .once()
            .return_const(());

        let tweet = tweet(1);
        create_tweet(&tweets, &tweet.message).await;
    }

    #[tokio::test]
    async fn test_delete_tweet() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| Some(tweet(1)));
        tweets.expect_delete().once().return_const(());
        delete_tweet(&tweets, 1).await;
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| None);
        tweets.expect_delete().once().return_const(());
        delete_tweet(&tweets, -1).await;
    }
}

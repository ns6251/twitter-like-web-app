use askama::Template;

use super::Tweet;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub tweets: Vec<Tweet>,
}

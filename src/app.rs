use std::env;
use std::fmt;

use reqwest;

use tokio;

use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct App<'a> {
    url: &'a str,
    access_token: String,
    page_id: String,
    booru_url: &'a str,
    tag: &'a str,
    rating: &'a str,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            url: "https://graph.facebook.com",
            access_token: env::var("ACCESS_TOKEN").unwrap(),
            page_id: env::var("PAGE_ID").unwrap(),
            booru_url: "https://safebooru.donmai.us",
            tag: "yasaka_kanako",
            rating: "safe",
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    VarError(env::VarError),
    RequestError(reqwest::Error),
    AsyncError(tokio::io::Error),
}

impl From<env::VarError> for ErrorKind {
    fn from(error: env::VarError) -> Self {
        Self::VarError(error)
    }
}

impl From<reqwest::Error> for ErrorKind {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}

impl From<tokio::io::Error> for ErrorKind {
    fn from(error: tokio::io::Error) -> Self {
        Self::AsyncError(error)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match &self {
            Self::VarError(err) => err.to_string(),
            Self::RequestError(err) => err.to_string(),
            Self::AsyncError(err) => err.to_string(),
        };

        write!(f, "{}", error)
    }
}

#[derive(Deserialize, Debug)]
pub struct BooruData {
    pub id: usize,
    pub large_file_url: String,
    pub tag_string_artist: String,
    pub source: String,
}

#[derive(Deserialize, Debug)]
pub struct PostData {
    pub post_id: String,
}

impl<'a> App<'a> {
    pub async fn post(&self) -> Result<(), ErrorKind> {

        let client = reqwest::Client::new();

        let req = client.get(format!("{}/posts.json", self.booru_url))
        .query(&[("tags", format!("{} rating:{} random:1", self.tag, self.rating)), ("limit", 1.to_string())])
        .send()
        .await?;

        let response = req.json::<Vec<BooruData>>().await?;

        let data = response.first().unwrap();

        let post = format!("{}/posts/{}", self.booru_url, data.id);

        let image_url = &data.large_file_url;

        let artist = &data.tag_string_artist;

        let source = &data.source;

        let req = client.post(format!("{}/{}/photos", self.url, self.page_id))
        .query(&[("access_token", self.access_token.to_string()), ("message", "Beautiful Goddess!".to_string()), ("url", image_url.to_string())])
        .send()
        .await?;

        let post_data = req.json::<PostData>().await?;

        let post_id = post_data.post_id;

        let comment_detail = format!(
r#"
Artist: {}
Source: {}
Booru: {}
"#, artist, source, post,
        );

        client.post(format!("{}/{}/comments", self.url, &post_id))
        .query(&[("access_token", self.access_token.to_string()), ("message", comment_detail), ("post_id", post_id)])
        .send()
        .await?;

        Ok(())
    }
}

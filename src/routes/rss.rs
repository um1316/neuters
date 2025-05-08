use crate::api::{error::ApiResult, search::fetch_articles_by_search, article::fetch_article_by_url};
use crate::client::Client;
use maud::html;
use chrono::{DateTime, Duration, Utc};

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

fn get_article_content(client: &Client, url: &str) -> Option<String> {
    match fetch_article_by_url(client, url) {
        Ok(article) => {
            let mut content = String::new();
            if let Some(elements) = article.content_elements {
                for element in elements.iter() {
                    if let Some(content_type) = element["type"].as_str() {
                        match content_type {
                            "paragraph" => {
                                if let Some(text) = element["content"].as_str() {
                                    content.push_str(&format!("<p>{}</p>", escape_xml(text)));
                                }
                            }
                            "header" => {
                                if let Some(text) = element["content"].as_str() {
                                    let level = element["level"].as_u64().unwrap_or(1);
                                    content.push_str(&format!("<h{}>{}</h{}>", level, escape_xml(text), level));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Some(content)
        }
        Err(_) => None
    }
}

pub fn render_rss(client: &Client) -> ApiResult<String> {
    // Use a more specific search query to get recent articles
    let articles = fetch_articles_by_search(client, "type:article", 0, 50)?;

    // Calculate the cutoff time (5 minutes ago)
    let cutoff_time = Utc::now() - Duration::minutes(5);

    let rss_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
        html! {
            rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" {
                channel {
                    title { (escape_xml("Neuters - Reuters Proxy - Latest Articles")) }
                    link { "https://neuters.example.com" }
                    description { (escape_xml("Latest articles from Neuters (last 5 minutes)")) }
                    language { "en" }
                    lastBuildDate { (Utc::now().to_rfc2822()) }
                    @if let Some(articles) = articles.articles {
                        @for article in articles.iter() {
                            @if let Ok(pub_time) = DateTime::parse_from_rfc3339(&article.published_time) {
                                @if pub_time >= cutoff_time {
                                    item {
                                        title { (escape_xml(&article.title)) }
                                        link { (escape_xml(&article.canonical_url)) }
                                        description { (escape_xml(&article.description)) }
                                        @if let Some(full_content) = get_article_content(client, &article.canonical_url) {
                                            content:encoded { (maud::PreEscaped(full_content)) }
                                        }
                                        pubDate { (escape_xml(&article.published_time)) }
                                        guid { (escape_xml(&article.canonical_url)) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }.into_string()
    );

    Ok(rss_content)
} 
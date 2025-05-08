use crate::api::{
    article::fetch_article_by_url,
    error::ApiResult,
    section::fetch_articles_by_section
};
use crate::client::Client;
use serde_json::Value;

pub fn render_rss(client: &Client) -> ApiResult<String> {
    // Fetch the latest 20 articles using the section API instead of search API
    // Using "/world/" section as it's a main section that should exist
    let articles = fetch_articles_by_section(client, "/world/", 0, 20)?;

    // Create XML manually to avoid any formatting issues
    let mut rss_content = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    rss_content.push_str("<rss version=\"2.0\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\">\n");
    rss_content.push_str("  <channel>\n");
    rss_content.push_str("    <title>Neuters - Reuters Proxy</title>\n");
    rss_content.push_str("    <link>https://neuters.example.com</link>\n");
    rss_content.push_str("    <description>Latest articles from Neuters</description>\n");
    
    if let Some(articles) = articles.articles {
        for article in articles.iter() {
            rss_content.push_str("    <item>\n");
            rss_content.push_str(&format!("      <title>{}</title>\n", escape_xml(&article.title)));
            rss_content.push_str(&format!("      <link>{}</link>\n", escape_xml(&article.canonical_url)));
            rss_content.push_str(&format!("      <description>{}</description>\n", escape_xml(&article.description)));
            rss_content.push_str(&format!("      <pubDate>{}</pubDate>\n", escape_xml(&article.published_time)));
            
            // For each article in the feed, try to fetch its full content
            if let Ok(full_article) = fetch_article_by_url(client, &article.canonical_url) {
                if let Some(content) = extract_article_content(&full_article.content_elements) {
                    rss_content.push_str(&format!("      <content:encoded><![CDATA[{}]]></content:encoded>\n", content));
                }
            }
            
            rss_content.push_str("    </item>\n");
        }
    }
    
    rss_content.push_str("  </channel>\n");
    rss_content.push_str("</rss>");

    Ok(rss_content)
}

// Helper function to escape XML special characters
fn escape_xml(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

// Extract article content from content elements
fn extract_article_content(content_elements: &Option<Box<[Value]>>) -> Option<String> {
    let elements = content_elements.as_ref()?;
    
    let mut content = String::new();
    
    for element in elements.iter() {
        match element.get("type").and_then(|t| t.as_str()) {
            Some("paragraph") => {
                if let Some(text) = element.get("content").and_then(|c| c.as_str()) {
                    content.push_str("<p>");
                    content.push_str(text);
                    content.push_str("</p>\n");
                }
            },
            Some("header") => {
                if let Some(text) = element.get("content").and_then(|c| c.as_str()) {
                    let level = element.get("level").and_then(|l| l.as_u64()).unwrap_or(2);
                    content.push_str(&format!("<h{level}>{}</h{level}>\n", text));
                }
            },
            Some("image") => {
                if let Some(url) = element.get("url").and_then(|u| u.as_str()) {
                    let alt = element.get("alt").and_then(|a| a.as_str()).unwrap_or("");
                    content.push_str(&format!("<img src=\"{}\" alt=\"{}\" />\n", url, alt));
                }
            },
            _ => {} // Skip other element types
        }
    }
    
    if content.is_empty() {
        None
    } else {
        Some(content)
    }
} 
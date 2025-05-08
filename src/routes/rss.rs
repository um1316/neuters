use crate::api::{error::ApiResult, section::fetch_articles_by_section};
use crate::client::Client;

pub fn render_rss(client: &Client) -> ApiResult<String> {
    // Fetch the latest 20 articles using the section API instead of search API
    // Using "/world/" section as it's a main section that should exist
    let articles = fetch_articles_by_section(client, "/world/", 0, 20)?;

    // Create XML manually to avoid any formatting issues
    let mut rss_content = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    rss_content.push_str("<rss version=\"2.0\">\n");
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
use crate::{
    api::{error::ApiResult, markit::fetch_by_stock_symbol},
    client::Client,
};

pub fn render_market(client: &Client, path: &str) -> ApiResult<String> {
    let company = if let Some(end) = path.find('/') {
        &path[..end]
    } else {
        path
    };

    let articles = fetch_by_stock_symbol(client, company)?;

    let document = crate::document! {
        company,
        maud::html! {
            company
            ul {
                @for article in articles.articles.iter() {
                    li { a href=(&article.canonical_url) { (&article.title) } }
                }
            }
        },

    };
    Ok(document.into_string())
}

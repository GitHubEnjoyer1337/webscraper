use reqwest::Error;
use scraper::{Html, Selector};
use futures::future::join_all;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls =  vec![
        "https://www.rust-lang.org",
        "https://www.crates.io",
        "https://www.docs.rs",
    ];

    let fetches = urls.iter().map(|&url| fetch_links(url));

    let results = join_all(fetches).await;

    for result in results {
        match result {
            Ok((url, links)) => {
                println!("Links from {}", url);
                for link in links {
                    println!("{}", link);
                }
                println!("--------------------");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}



async fn fetch_links(url: &str) -> Result<(String, Vec<String>), reqwest::Error> {
    // Send the GET request and await the response
    let response = reqwest::get(url).await?.error_for_status()?;

    // Get the response body text
    let body = response.text().await?;

    // Parse the HTML document
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    let mut links = Vec::new();

    // Extract links from the document
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(link.to_string());
        }
    }

    Ok((url.to_string(), links))
}






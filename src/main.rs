use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn check_internal_links(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::new().get(url).send()?;
    let body = response.text()?;
    let document = Html::parse_document(&body);

    let selectors = [
        Selector::parse("a[href='#']"),
        Selector::parse("iframe[src='#']"),
    ];

    for selector in &selectors {
        for link in document.select(&selector.as_ref().unwrap()) {
            let href = link.value().attr("href").unwrap_or("");
            if !document
                .select(&Selector::parse(href).unwrap())
                .next()
                .is_some()
            {
                println!("Broken internal link: {}", href);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://dark.elm.dmy.fr/packages/gicentre/elm-vegalite/latest/VegaLite";
    check_internal_links(url)?;
    Ok(())
}

use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter page: ");
    let mut num_page = String::new();

    io::stdin().read_line(&mut num_page).expect("Failed read to line");
    let num_page = num_page.trim();

    let client = reqwest::Client::builder().build()?;

    let response = client
        .get(format!("https://kino.mail.ru/cinema/all/?page={}", num_page))
        .send()
        .await?;

    if response.status() != 200 {
        println!("Page not found");
    }

    let filename = format!("products_{}.csv", num_page);
    let path = std::path::Path::new(&filename);
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["url", "name", "mail_rate", "imdb_rate"])
        .unwrap();

    let response = response.text().await?;

    let document = scraper::Html::parse_document(&response);
    let cols_selector = scraper::Selector::parse(r#"div[class="cols cols_percent cols_margin"]"#).unwrap();
    let wrapper_selector = scraper::Selector::parse(r#"div[class="cols__wrapper"]"#).unwrap();
    let child_selector = scraper::Selector::parse(r#".cols__column"#).unwrap();
    let title_selector = scraper::Selector::parse(r#"span[class="text text_block text_fixed text_light_large"]"#).unwrap();
    let href_selector = scraper::Selector::parse(r#"a.link"#).unwrap();
    let mail_rate_selector = scraper::Selector::parse(".p-rate-flag__text").unwrap();
    let imdb_rate_selector = scraper::Selector::parse(".p-rate-flag__imdb-text").unwrap();

    let table = document.select(&cols_selector).next().unwrap();
    let _cells = table.select(&wrapper_selector).next().unwrap();
    let childs = _cells.select(&child_selector);

    let domain = "https://kino.mail.ru".to_owned();
    for child in childs {
        let title_element = child.select(&title_selector).next().unwrap();

        let _title_name = title_element.text().collect::<String>();
        let _title_href_part = title_element.select(&href_selector).next().unwrap().attr("href").unwrap();
        let _title_href_full = format!("{domain}{_title_href_part}");
        let _mail_rate = child.select(&mail_rate_selector).next().unwrap().text().collect::<String>();
        let _imdb_rate = child.select(&imdb_rate_selector).next().unwrap().text().collect::<String>();
        
        writer.write_record(&[_title_href_full, _title_name, _mail_rate, _imdb_rate]).unwrap();
    }

    writer.flush().unwrap();
    Ok(())
}
